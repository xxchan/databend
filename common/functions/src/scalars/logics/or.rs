// Copyright 2022 Datafuse Labs.
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

use std::sync::Arc;

use common_datavalues::prelude::*;
use common_exception::Result;

use super::logic::LogicExpression;
use super::logic::LogicFunctionImpl;
use super::logic::LogicOperator;
use crate::calcute;
use crate::impl_logic_expression;
use crate::scalars::cast_column_field;
use crate::scalars::Function;
use crate::scalars::FunctionFeatures;
use crate::scalars::TypedFunctionDescription;

impl_logic_expression!(LogicOrExpression, |, |lhs: bool, rhs: bool, lhs_v: bool, rhs_v: bool| -> (bool, bool) {
    (lhs | rhs,  (lhs_v & rhs_v) | (lhs | rhs))
});

#[derive(Clone)]
pub struct LogicOrFunction;

impl LogicOrFunction {
    pub fn try_create(_display_name: &str, args: &[&DataTypePtr]) -> Result<Box<dyn Function>> {
        LogicFunctionImpl::<LogicOrExpression>::try_create(LogicOperator::Or, args)
    }

    pub fn desc() -> TypedFunctionDescription {
        TypedFunctionDescription::creator(Box::new(Self::try_create)).features(
            FunctionFeatures::default()
                .deterministic()
                .disable_passthrough_null()
                .num_arguments(2),
        )
    }
}
