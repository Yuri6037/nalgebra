use std::ops::{
    Mul,
    MulAssign,
    Div,
    DivAssign
};

use crate::OScaling;
use crate::{ClosedDiv, ClosedMul, DefaultAllocator, DimName, OVector, allocator::Allocator, Scalar, OPoint};

impl<T, D: DimName> Mul<OVector<T, D>> for OScaling<T, D>
    where T: Scalar + ClosedMul, DefaultAllocator: Allocator<T, D>
{
    type Output = OScaling<T, D>;

    fn mul(self, rhs: OVector<T, D>) -> Self::Output
    {
        return OScaling::<T, D>(self.0.component_mul(&rhs));
    }
}

impl<T, D: DimName> MulAssign<OVector<T, D>> for OScaling<T, D>
    where T: Scalar + ClosedMul, DefaultAllocator: Allocator<T, D>
{
    fn mul_assign(&mut self, rhs: OVector<T, D>)
    {
        self.0.component_mul_assign(&rhs);
    }
}

impl<T, D: DimName> Div<OVector<T, D>> for OScaling<T, D>
    where T: Scalar + ClosedDiv, DefaultAllocator: Allocator<T, D>
{
    type Output = OScaling<T, D>;

    fn div(self, rhs: OVector<T, D>) -> Self::Output
    {
        return OScaling::<T, D>(self.0.component_div(&rhs));
    }
}

impl<T, D: DimName> DivAssign<OVector<T, D>> for OScaling<T, D>
    where T: Scalar + ClosedDiv, DefaultAllocator: Allocator<T, D>
{
    fn div_assign(&mut self, rhs: OVector<T, D>)
    {
        self.0.component_div_assign(&rhs);
    }
}

impl<T, D: DimName> Mul<OScaling<T, D>> for OScaling<T, D>
    where T: Scalar + ClosedMul, DefaultAllocator: Allocator<T, D>
{
    type Output = OScaling<T, D>;

    fn mul(self, rhs: OScaling<T, D>) -> Self::Output
    {
        return OScaling::<T, D>(self.0.component_mul(&rhs.0));
    }
}

impl<T, D: DimName> MulAssign<OScaling<T, D>> for OScaling<T, D>
    where T: Scalar + ClosedMul, DefaultAllocator: Allocator<T, D>
{
    fn mul_assign(&mut self, rhs: OScaling<T, D>)
    {
        self.0.component_mul_assign(&rhs.0);
    }
}

impl<T, D: DimName> Div<OScaling<T, D>> for OScaling<T, D>
    where T: Scalar + ClosedDiv, DefaultAllocator: Allocator<T, D>
{
    type Output = OScaling<T, D>;

    fn div(self, rhs: OScaling<T, D>) -> Self::Output
    {
        return OScaling::<T, D>(self.0.component_div(&rhs.0));
    }
}

impl<T, D: DimName> DivAssign<OScaling<T, D>> for OScaling<T, D>
    where T: Scalar + ClosedDiv, DefaultAllocator: Allocator<T, D>
{
    fn div_assign(&mut self, rhs: OScaling<T, D>)
    {
        self.0.component_div_assign(&rhs.0);
    }
}

impl<T, D: DimName> Mul<OPoint<T, D>> for OScaling<T, D>
    where T: Scalar + ClosedMul, DefaultAllocator: Allocator<T, D>
{
    type Output = OPoint<T, D>;

    fn mul(self, rhs: OPoint<T, D>) -> Self::Output
    {
        return OPoint::from(self.0.component_mul(&rhs.coords));
    }
}

impl<T, D: DimName> Div<OPoint<T, D>> for OScaling<T, D>
    where T: Scalar + ClosedDiv, DefaultAllocator: Allocator<T, D>
{
    type Output = OPoint<T, D>;

    fn div(self, rhs: OPoint<T, D>) -> Self::Output
    {
        return OPoint::from(self.0.component_div(&rhs.coords));
    }
}
