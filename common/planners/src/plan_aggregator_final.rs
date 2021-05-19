// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::sync::Arc;

use common_datavalues::DataSchemaRef;

use crate::ExpressionAction;
use crate::PlanNode;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AggregatorFinalPlan {
    pub aggr_expr: Vec<ExpressionAction>,
    pub group_expr: Vec<ExpressionAction>,
    pub schema: DataSchemaRef,
    pub input: Arc<PlanNode>
}

impl AggregatorFinalPlan {
    pub fn schema(&self) -> DataSchemaRef {
        self.schema.clone()
    }

    pub fn input(&self) -> Arc<PlanNode> {
        self.input.clone()
    }

    pub fn set_input(&mut self, input: &PlanNode) {
        self.input = Arc::new(input.clone());
    }
}
