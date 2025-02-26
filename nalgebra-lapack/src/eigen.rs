#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

use num::Zero;
use num_complex::Complex;

use simba::scalar::RealField;

use crate::ComplexHelper;
use na::allocator::Allocator;
use na::dimension::{Const, Dim};
use na::{DefaultAllocator, Matrix, OMatrix, OVector, Scalar};

use lapack;

/// Eigendecomposition of a real square matrix with real eigenvalues.
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde-serialize",
    serde(
        bound(serialize = "DefaultAllocator: Allocator<T, D, D> + Allocator<T, D>,
         OVector<T, D>: Serialize,
         OMatrix<T, D, D>: Serialize")
    )
)]
#[cfg_attr(
    feature = "serde-serialize",
    serde(
        bound(deserialize = "DefaultAllocator: Allocator<T, D, D> + Allocator<T, D>,
         OVector<T, D>: Serialize,
         OMatrix<T, D, D>: Deserialize<'de>")
    )
)]
#[derive(Clone, Debug)]
pub struct Eigen<T: Scalar, D: Dim>
where
    DefaultAllocator: Allocator<T, D> + Allocator<T, D, D>,
{
    /// The eigenvalues of the decomposed matrix.
    pub eigenvalues: OVector<T, D>,
    /// The (right) eigenvectors of the decomposed matrix.
    pub eigenvectors: Option<OMatrix<T, D, D>>,
    /// The left eigenvectors of the decomposed matrix.
    pub left_eigenvectors: Option<OMatrix<T, D, D>>,
}

impl<T: Scalar + Copy, D: Dim> Copy for Eigen<T, D>
where
    DefaultAllocator: Allocator<T, D> + Allocator<T, D, D>,
    OVector<T, D>: Copy,
    OMatrix<T, D, D>: Copy,
{
}

impl<T: EigenScalar + RealField, D: Dim> Eigen<T, D>
where
    DefaultAllocator: Allocator<T, D, D> + Allocator<T, D>,
{
    /// Computes the eigenvalues and eigenvectors of the square matrix `m`.
    ///
    /// If `eigenvectors` is `false` then, the eigenvectors are not computed explicitly.
    pub fn new(
        mut m: OMatrix<T, D, D>,
        left_eigenvectors: bool,
        eigenvectors: bool,
    ) -> Option<Eigen<T, D>> {
        assert!(
            m.is_square(),
            "Unable to compute the eigenvalue decomposition of a non-square matrix."
        );

        let ljob = if left_eigenvectors { b'V' } else { b'T' };
        let rjob = if eigenvectors { b'V' } else { b'T' };

        let (nrows, ncols) = m.shape_generic();
        let n = nrows.value();

        let lda = n as i32;

        // TODO: avoid the initialization?
        let mut wr = Matrix::zeros_generic(nrows, Const::<1>);
        // TODO: Tap into the workspace.
        let mut wi = Matrix::zeros_generic(nrows, Const::<1>);

        let mut info = 0;
        let mut placeholder1 = [T::zero()];
        let mut placeholder2 = [T::zero()];

        let lwork = T::xgeev_work_size(
            ljob,
            rjob,
            n as i32,
            m.as_mut_slice(),
            lda,
            wr.as_mut_slice(),
            wi.as_mut_slice(),
            &mut placeholder1,
            n as i32,
            &mut placeholder2,
            n as i32,
            &mut info,
        );

        lapack_check!(info);

        let mut work = vec![T::zero(); lwork as usize];

        match (left_eigenvectors, eigenvectors) {
            (true, true) => {
                // TODO: avoid the initializations?
                let mut vl = Matrix::zeros_generic(nrows, ncols);
                let mut vr = Matrix::zeros_generic(nrows, ncols);

                T::xgeev(
                    ljob,
                    rjob,
                    n as i32,
                    m.as_mut_slice(),
                    lda,
                    wr.as_mut_slice(),
                    wi.as_mut_slice(),
                    &mut vl.as_mut_slice(),
                    n as i32,
                    &mut vr.as_mut_slice(),
                    n as i32,
                    &mut work,
                    lwork,
                    &mut info,
                );
                lapack_check!(info);

                if wi.iter().all(|e| e.is_zero()) {
                    return Some(Self {
                        eigenvalues: wr,
                        left_eigenvectors: Some(vl),
                        eigenvectors: Some(vr),
                    });
                }
            }
            (true, false) => {
                // TODO: avoid the initialization?
                let mut vl = Matrix::zeros_generic(nrows, ncols);

                T::xgeev(
                    ljob,
                    rjob,
                    n as i32,
                    m.as_mut_slice(),
                    lda,
                    wr.as_mut_slice(),
                    wi.as_mut_slice(),
                    &mut vl.as_mut_slice(),
                    n as i32,
                    &mut placeholder2,
                    1 as i32,
                    &mut work,
                    lwork,
                    &mut info,
                );
                lapack_check!(info);

                if wi.iter().all(|e| e.is_zero()) {
                    return Some(Self {
                        eigenvalues: wr,
                        left_eigenvectors: Some(vl),
                        eigenvectors: None,
                    });
                }
            }
            (false, true) => {
                // TODO: avoid the initialization?
                let mut vr = Matrix::zeros_generic(nrows, ncols);

                T::xgeev(
                    ljob,
                    rjob,
                    n as i32,
                    m.as_mut_slice(),
                    lda,
                    wr.as_mut_slice(),
                    wi.as_mut_slice(),
                    &mut placeholder1,
                    1 as i32,
                    &mut vr.as_mut_slice(),
                    n as i32,
                    &mut work,
                    lwork,
                    &mut info,
                );
                lapack_check!(info);

                if wi.iter().all(|e| e.is_zero()) {
                    return Some(Self {
                        eigenvalues: wr,
                        left_eigenvectors: None,
                        eigenvectors: Some(vr),
                    });
                }
            }
            (false, false) => {
                T::xgeev(
                    ljob,
                    rjob,
                    n as i32,
                    m.as_mut_slice(),
                    lda,
                    wr.as_mut_slice(),
                    wi.as_mut_slice(),
                    &mut placeholder1,
                    1 as i32,
                    &mut placeholder2,
                    1 as i32,
                    &mut work,
                    lwork,
                    &mut info,
                );
                lapack_check!(info);

                if wi.iter().all(|e| e.is_zero()) {
                    return Some(Self {
                        eigenvalues: wr,
                        left_eigenvectors: None,
                        eigenvectors: None,
                    });
                }
            }
        }

        None
    }

    /// The complex eigenvalues of the given matrix.
    ///
    /// Panics if the eigenvalue computation does not converge.
    pub fn complex_eigenvalues(mut m: OMatrix<T, D, D>) -> OVector<Complex<T>, D>
    where
        DefaultAllocator: Allocator<Complex<T>, D>,
    {
        assert!(
            m.is_square(),
            "Unable to compute the eigenvalue decomposition of a non-square matrix."
        );

        let nrows = m.shape_generic().0;
        let n = nrows.value();

        let lda = n as i32;

        // TODO: avoid the initialization?
        let mut wr = Matrix::zeros_generic(nrows, Const::<1>);
        let mut wi = Matrix::zeros_generic(nrows, Const::<1>);

        let mut info = 0;
        let mut placeholder1 = [T::zero()];
        let mut placeholder2 = [T::zero()];

        let lwork = T::xgeev_work_size(
            b'T',
            b'T',
            n as i32,
            m.as_mut_slice(),
            lda,
            wr.as_mut_slice(),
            wi.as_mut_slice(),
            &mut placeholder1,
            n as i32,
            &mut placeholder2,
            n as i32,
            &mut info,
        );

        lapack_panic!(info);

        let mut work = vec![T::zero(); lwork as usize];

        T::xgeev(
            b'T',
            b'T',
            n as i32,
            m.as_mut_slice(),
            lda,
            wr.as_mut_slice(),
            wi.as_mut_slice(),
            &mut placeholder1,
            1 as i32,
            &mut placeholder2,
            1 as i32,
            &mut work,
            lwork,
            &mut info,
        );
        lapack_panic!(info);

        let mut res = Matrix::zeros_generic(nrows, Const::<1>);

        for i in 0..res.len() {
            res[i] = Complex::new(wr[i], wi[i]);
        }

        res
    }

    /// The determinant of the decomposed matrix.
    #[inline]
    #[must_use]
    pub fn determinant(&self) -> T {
        let mut det = T::one();
        for e in self.eigenvalues.iter() {
            det *= *e;
        }

        det
    }
}

/*
 *
 * Lapack functions dispatch.
 *
 */
/// Trait implemented by scalar type for which Lapack function exist to compute the
/// eigendecomposition.
pub trait EigenScalar: Scalar {
    #[allow(missing_docs)]
    fn xgeev(
        jobvl: u8,
        jobvr: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vl: &mut [Self],
        ldvl: i32,
        vr: &mut [Self],
        ldvr: i32,
        work: &mut [Self],
        lwork: i32,
        info: &mut i32,
    );
    #[allow(missing_docs)]
    fn xgeev_work_size(
        jobvl: u8,
        jobvr: u8,
        n: i32,
        a: &mut [Self],
        lda: i32,
        wr: &mut [Self],
        wi: &mut [Self],
        vl: &mut [Self],
        ldvl: i32,
        vr: &mut [Self],
        ldvr: i32,
        info: &mut i32,
    ) -> i32;
}

macro_rules! real_eigensystem_scalar_impl (
    ($N: ty, $xgeev: path) => (
        impl EigenScalar for $N {
            #[inline]
            fn xgeev(jobvl: u8, jobvr: u8, n: i32, a: &mut [Self], lda: i32,
                     wr: &mut [Self], wi: &mut [Self],
                     vl: &mut [Self], ldvl: i32, vr: &mut [Self], ldvr: i32,
                     work: &mut [Self], lwork: i32, info: &mut i32) {
                unsafe { $xgeev(jobvl, jobvr, n, a, lda, wr, wi, vl, ldvl, vr, ldvr, work, lwork, info) }
            }


            #[inline]
            fn xgeev_work_size(jobvl: u8, jobvr: u8, n: i32, a: &mut [Self], lda: i32,
                               wr: &mut [Self], wi: &mut [Self], vl: &mut [Self], ldvl: i32,
                               vr: &mut [Self], ldvr: i32, info: &mut i32) -> i32 {
                let mut work = [ Zero::zero() ];
                let lwork = -1 as i32;

                unsafe { $xgeev(jobvl, jobvr, n, a, lda, wr, wi, vl, ldvl, vr, ldvr, &mut work, lwork, info) };
                ComplexHelper::real_part(work[0]) as i32
            }
        }
    )
);

real_eigensystem_scalar_impl!(f32, lapack::sgeev);
real_eigensystem_scalar_impl!(f64, lapack::dgeev);

//// TODO: decomposition of complex matrix and matrices with complex eigenvalues.
// eigensystem_complex_impl!(f32, lapack::cgeev);
// eigensystem_complex_impl!(f64, lapack::zgeev);
