// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! Enforces a circuit expression in a compiled Leo program.

use crate::errors::ExpressionError;
use crate::program::ConstrainedProgram;
use crate::value::ConstrainedCircuitMember;
use crate::value::ConstrainedValue;
use crate::GroupType;
use leo_asg::CircuitInitExpression;
use leo_asg::CircuitMember;
use leo_asg::Span;

use snarkvm_models::curves::PrimeField;
use snarkvm_models::gadgets::r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn enforce_circuit<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expr: &CircuitInitExpression<'a>,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>, ExpressionError> {
        let circuit = expr.circuit.get();
        let members = circuit.members.borrow();

        let mut resolved_members = Vec::with_capacity(members.len());

        // type checking is already done in asg
        for (name, inner) in expr.values.iter() {
            let target = members
                .get(&name.name)
                .expect("illegal name in asg circuit init expression");
            match target {
                CircuitMember::Variable(_type_) => {
                    let variable_value = self.enforce_expression(cs, inner.get())?;
                    resolved_members.push(ConstrainedCircuitMember(name.clone(), variable_value));
                }
                _ => return Err(ExpressionError::expected_circuit_member(name.to_string(), span.clone())),
            }
        }

        let value = ConstrainedValue::CircuitExpression(circuit, resolved_members);
        Ok(value)
    }
}
