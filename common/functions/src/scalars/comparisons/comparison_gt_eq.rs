// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::cmp::Ordering;

use common_arrow::arrow::compute::comparison::Simd8;
use common_arrow::arrow::compute::comparison::Simd8PartialOrd;
use common_datavalues::prelude::*;
use num::traits::AsPrimitive;

use super::comparison::ComparisonFunctionCreator;
use super::comparison::ComparisonImpl;
use super::comparison_lt_eq::BooleanSimdLtEq;
use super::utils::*;
use crate::scalars::EvalContext;

pub type ComparisonGtEqFunction = ComparisonFunctionCreator<ComparisonGtEqImpl>;

#[derive(Clone)]
pub struct ComparisonGtEqImpl;

impl ComparisonImpl for ComparisonGtEqImpl {
    type BooleanSimd = BooleanSimdGtEq;

    fn eval_simd<T>(l: T::Simd, r: T::Simd) -> u8
    where
        T: PrimitiveType + Simd8,
        T::Simd: Simd8PartialOrd,
    {
        l.gt_eq(r)
    }

    fn eval_primitive<L, R, M>(l: L::RefType<'_>, r: R::RefType<'_>, _ctx: &mut EvalContext) -> bool
    where
        L: PrimitiveType + AsPrimitive<M>,
        R: PrimitiveType + AsPrimitive<M>,
        M: PrimitiveType,
    {
        l.to_owned_scalar().as_().ge(&r.to_owned_scalar().as_())
    }

    fn eval_binary(l: &[u8], r: &[u8], _ctx: &mut EvalContext) -> bool {
        l >= r
    }

    fn eval_variant(l: &VariantValue, r: &VariantValue, _ctx: &mut EvalContext) -> bool {
        let result = l.cmp(r);
        result == Ordering::Equal || result == Ordering::Greater
    }
}

#[derive(Clone)]
pub struct BooleanSimdGtEq;

impl BooleanSimdImpl for BooleanSimdGtEq {
    fn vector_vector(lhs: &BooleanColumn, rhs: &BooleanColumn) -> BooleanColumn {
        CommonBooleanOp::compare_op(lhs, rhs, |a, b| a | !b)
    }

    fn vector_const(lhs: &BooleanColumn, rhs: bool) -> BooleanColumn {
        if rhs {
            lhs.clone()
        } else {
            let all_ones = !0;
            CommonBooleanOp::compare_op_scalar(lhs, rhs, |_, _| all_ones)
        }
    }

    fn const_vector(lhs: bool, rhs: &BooleanColumn) -> BooleanColumn {
        BooleanSimdLtEq::vector_const(rhs, lhs)
    }
}
