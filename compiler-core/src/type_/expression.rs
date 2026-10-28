use crate::{ast::{TypedExpr, UntypedExpr, statement::{Stmt, StmtKind}}, type_::{Type, Typed, environment::Environment}};
use std::{sync::Arc, collections::HashMap};

#[derive(Debug)]
pub(crate) struct ExprTyper<'a, 'b> {
    environment: &'a mut Environment<'b>,
}

impl<'a, 'b> ExprTyper<'a, 'b> {
    pub fn new(
        environment: &'a mut Environment<'b>,
    ) -> Self {
        Self {
            environment
        }
    }

    /*
    pub(crate) fn instantiate(
        &mut self,
        t: Arc<Type>,
        ids: &mut imbl::HashMap<u64, Arc<Type>>,
    ) -> Arc<Type> {
        self.environment.instantiate(t, ids, &self.hydrator)
    }

    pub fn new_unbound_var(&mut self) -> Arc<Type> {
        self.environment.new_unbound_var()
    }

    pub fn infer_or_error(&mut self, expr: UntypedExpr) -> Result<TypedExpr, Error> {
        if self.previous_panics {
            self.warn_for_unreachable_code(expr.location(), PanicPosition::PreviousExpression);
        }

        match expr {
            UntypedExpr::Todo {
                location,
                message: label,
                kind,
                ..
            } => Ok(self.infer_todo(location, kind, label)),

            UntypedExpr::Panic {
                location, message, ..
            } => Ok(self.infer_panic(location, message)),

            UntypedExpr::Echo {
                location,
                keyword_end,
                expression,
                message,
            } => Ok(self.infer_echo(location, keyword_end, expression, message)),

            UntypedExpr::Var { location, name, .. } => self.infer_var(
                name,
                location,
                ValueUsage::Other,
                ReferenceRegistration::Register,
            ),

            UntypedExpr::Int {
                location,
                value,
                int_value,
                ..
            } => {
                if self.environment.target == Target::JavaScript
                    && !self.current_function_definition.has_javascript_external
                {
                    check_javascript_int_safety(&int_value, location, self.problems);
                }

                Ok(self.infer_int(value, int_value, location))
            }

            UntypedExpr::Block {
                statements,
                location,
            } => Ok(self.infer_block(statements, location)),

            UntypedExpr::Tuple {
                location, elements, ..
            } => Ok(self.infer_tuple(elements, location)),

            UntypedExpr::Float {
                location,
                value,
                float_value,
            } => {
                check_float_safety(float_value, location, self.problems);
                Ok(self.infer_float(value, float_value, location))
            }

            UntypedExpr::String {
                location, value, ..
            } => Ok(self.infer_string(value, location)),

            UntypedExpr::Pipeline { expressions } => Ok(self.infer_pipeline(expressions)),

            UntypedExpr::Fn {
                location,
                kind,
                arguments,
                body,
                return_annotation,
                ..
            } => Ok(self.infer_fn(arguments, &[], body, kind, return_annotation, location)),

            UntypedExpr::Case {
                location,
                subjects,
                clauses,
                ..
            } => Ok(self.infer_case(subjects, clauses, location)),

            UntypedExpr::List {
                location,
                elements,
                tail,
            } => Ok(self.infer_list(elements, tail, location)),

            UntypedExpr::Call {
                location,
                fun,
                arguments,
                open_parenthesis,
                ..
            } => Ok(self.infer_call(
                *fun,
                arguments,
                location,
                Some(open_parenthesis),
                CallKind::Function,
            )),

            UntypedExpr::BinOp {
                location,
                operator,
                operator_start,
                left,
                right,
            } => Ok(self.infer_binop(operator, operator_start, *left, *right, location)),

            UntypedExpr::FieldAccess {
                label_location,
                label,
                container,
                location,
            } => Ok(self.infer_field_access(
                *container,
                location,
                label,
                label_location,
                FieldAccessUsage::Other,
            )),

            UntypedExpr::TupleIndex {
                location,
                index,
                tuple,
                ..
            } => self.infer_tuple_index(*tuple, index, location),

            UntypedExpr::BitArray { location, segments } => {
                self.infer_bit_array(segments, location)
            }

            UntypedExpr::RecordUpdate {
                location,
                spread_start,
                constructor,
                record,
                arguments,
            } => self.infer_record_update(*constructor, record, arguments, location, spread_start),

            UntypedExpr::NegateBool { location, value } => {
                Ok(self.infer_negate_bool(location, *value))
            }

            UntypedExpr::NegateInt { location, value } => {
                Ok(self.infer_negate_int(location, *value))
            }
        }
    }

    fn infer_string(&mut self, value: EcoString, location: SrcSpan) -> TypedExpr {
        TypedExpr::String {
            location,
            value,
            type_: string(),
        }
    }

    fn infer_int(&mut self, value: EcoString, int_value: BigInt, location: SrcSpan) -> TypedExpr {
        TypedExpr::Int {
            location,
            value,
            int_value,
            type_: int(),
        }
    }

    fn infer_float(
        &mut self,
        value: EcoString,
        float_value: LiteralFloatValue,
        location: SrcSpan,
    ) -> TypedExpr {
        TypedExpr::Float {
            location,
            value,
            float_value,
            type_: float(),
        }
    }

    pub(crate) fn infer_statements(
        &mut self,
        untyped: Vec<UntypedStatement>,
    ) -> Vec<TypedStatement> {
        let count = untyped.len();
        let location = SrcSpan::new(
            untyped.first().location().start,
            untyped.last().location().end,
        );
        self.infer_iter_statements(location, count, untyped.into_iter())
    }

    fn infer_iter_statements<I>(
        &mut self,
        location: SrcSpan,
        count: usize,
        untyped: StatementsIter,
    ) -> Vec<TypedStatement>
    where I: IntoIterator<Item = UntypedStatement>
    {
        let mut statements: Vec<TypedStatement> = Vec::with_capacity(count);

        for (i, statement) in untyped.into_iter().enumerate() {
            match statement {
                Statement::Use(use_) => {
                    let statement = self.infer_use(use_, location, untyped.collect());
                    statements.push(statement);
                    break; // Inferring the use has consumed the rest of the exprs
                }
                Statement::Expression(expression) => {
                    let location = expression.location();
                    let expression = match self.infer_or_error(expression) {
                        Ok(expression) => expression,
                        Err(error) => {
                            self.problems.error(error);
                            self.error_expr(location)
                        }
                    };

                    // This isn't the final expression in the sequence, so call the
                    // `expression_discarded` function to see if anything is being
                    // discarded that we think shouldn't be.
                    if i < count {
                        self.expression_discarded(&expression);
                    }
                    statements.push(Statement::Expression(expression));
                }
                Statement::Assignment(assignment) => {
                    let assignment = self.infer_assignment(*assignment);
                    statements.push(Statement::Assignment(Box::new(assignment)));
                }
                Statement::Assert(assert) => {
                    let assert = self.infer_assert(assert);
                    statements.push(Statement::Assert(assert));
                }
            }
        }

        statements
    }

    fn infer_fn(
        &mut self,
        arguments: Vec<UntypedArg>,
        expected_arguments: &[Arc<Type>],
        body: Vec1<UntypedStatement>,
        kind: FunctionLiteralKind,
        return_annotation: Option<TypeAst>,
        location: SrcSpan,
    ) -> TypedExpr {
        for Arg { names, .. } in arguments.iter() {
            check_argument_names(names, self.problems);
        }

        let already_warned_for_unreachable_code = self.already_warned_for_unreachable_code;
        self.already_warned_for_unreachable_code = false;
        self.previous_panics = false;

        let outer_purity = self.purity;

        // If an anonymous function can panic, that doesn't mean that the outer
        // function can too, so we track the purity separately. For example, in
        // this code:
        //
        // ```gleam
        // pub fn divide_partial(dividend: Int) {
        //   fn(divisor) {
        //     case divisor {
        //       0 -> panic as "Cannot divide by 0"
        //       _ -> dividend / divisor
        //     }
        //   }
        // }
        // ```
        //
        // Although the `divide_partial` function uses the `panic` keyword, it is
        // actually pure. Only the anonymous function that it constructs is impure;
        // constructing and returning it does not have any side effects, so there is
        // no way for a call to `divide_partial` to produce any side effects.
        self.purity = Purity::Pure;

        let (arguments, body) = match self.do_infer_fn(
            None,
            arguments,
            expected_arguments,
            body,
            &return_annotation,
        ) {
            Ok(result) => result,
            Err(error) => {
                self.problems.error(error);
                return self.error_expr(location);
            }
        };
        let arguments_types = arguments.iter().map(|a| a.type_.clone()).collect();
        let type_ = fn_(arguments_types, body.last().type_());

        // Defining an anonymous function never panics.
        self.already_warned_for_unreachable_code = already_warned_for_unreachable_code;
        self.previous_panics = false;

        let function_purity = self.purity;
        self.purity = outer_purity;

        TypedExpr::Fn {
            location,
            type_,
            kind,
            arguments,
            body,
            return_annotation,
            purity: function_purity,
        }
    }

    fn infer_arg(
        &mut self,
        arg: UntypedArg,
        expected: Option<Arc<Type>>,
    ) -> Result<TypedArg, Error> {
        let Arg {
            names,
            annotation,
            location,
            ..
        } = arg;
        let type_ = annotation
            .clone()
            .map(|type_| self.type_from_ast(&type_))
            .unwrap_or_else(|| Ok(self.new_unbound_var()))?;

        match &names {
            ArgNames::Named { .. } | ArgNames::NamedLabelled { .. } => (),
            ArgNames::Discard { name, .. } | ArgNames::LabelledDiscard { name, .. } => {
                let _ = self
                    .environment
                    .discarded_names
                    .insert(name.clone(), location);
            }
        }

        // If we know the expected type of the argument from its contextual
        // usage then unify the newly constructed type with the expected type.
        // We do this here because then there is more type information for the
        // function being type checked, resulting in better type errors and the
        // record field access syntax working.
        if let Some(expected) = expected {
            unify(expected, type_.clone()).map_err(|error| convert_unify_error(error, location))?;
        }

        Ok(Arg {
            names,
            location,
            annotation,
            type_,
        })
    }

    fn infer_call(
        &mut self,
        fun: UntypedExpr,
        arguments: Vec<CallArg<UntypedExpr>>,
        location: SrcSpan,
        open_parenthesis: Option<u32>,
        kind: CallKind,
    ) -> TypedExpr {
        let (fun, arguments, type_) = self.do_infer_call(fun, arguments, location, kind);

        // One common mistake is to think that the syntax for adding a message
        // to a `todo` or a `panic` exception is to `todo("...")`, but really
        // this does nothing as the `todo` or `panic` throws the exception
        // before it gets to the function call `("...")`.
        // If we find code doing this then emit a warning.
        let todopanic = match fun {
            TypedExpr::Todo { .. } => Some((location, TodoOrPanic::Todo)),
            TypedExpr::Panic { .. } => Some((location, TodoOrPanic::Panic)),
            TypedExpr::Int { .. }
            | TypedExpr::Float { .. }
            | TypedExpr::String { .. }
            | TypedExpr::Block { .. }
            | TypedExpr::Pipeline { .. }
            | TypedExpr::Var { .. }
            | TypedExpr::Fn { .. }
            | TypedExpr::List { .. }
            | TypedExpr::Call { .. }
            | TypedExpr::BinOp { .. }
            | TypedExpr::Case { .. }
            | TypedExpr::RecordAccess { .. }
            | TypedExpr::PositionalAccess { .. }
            | TypedExpr::ModuleSelect { .. }
            | TypedExpr::Tuple { .. }
            | TypedExpr::TupleIndex { .. }
            | TypedExpr::Echo { .. }
            | TypedExpr::BitArray { .. }
            | TypedExpr::RecordUpdate { .. }
            | TypedExpr::NegateBool { .. }
            | TypedExpr::NegateInt { .. }
            | TypedExpr::Invalid { .. } => None,
        };
        if let Some((location, kind)) = todopanic {
            let arguments_location = match (arguments.first(), arguments.last()) {
                (Some(first), Some(last)) => Some(SrcSpan {
                    start: first.location().start,
                    end: last.location().end,
                }),
                _ => None,
            };
            self.problems.warning(Warning::TodoOrPanicUsedAsFunction {
                kind,
                location,
                arguments_location,
                arguments: arguments.len(),
            });
        }

        self.purity = self.purity.merge(fun.called_function_purity());

        TypedExpr::Call {
            location,
            type_,
            arguments,
            open_parenthesis,
            fun: Box::new(fun),
        }
    }

    fn infer_array_creation(&mut self, elements: Vec<UntypedExpr>, location: SrcSpan) -> TypedExpr {
        let elements = elements
            .into_iter()
            .map(|element| self.infer(element))
            .collect_vec();
        let type_ = tuple(elements.iter().map(HasType::type_).collect_vec());
        TypedExpr::Tuple {
            location,
            elements,
            type_,
        }
    }

    fn infer_var(
        &mut self,
        name: EcoString,
        location: SrcSpan,
        value_usage: ValueUsage,
        register_reference: ReferenceRegistration,
    ) -> Result<TypedExpr, Error> {
        let constructor = self.do_infer_value_constructor(
            &None,
            &name,
            &location,
            value_usage,
            register_reference,
        )?;
        self.narrow_implementations(location, &constructor.variant)?;
        Ok(TypedExpr::Var {
            constructor,
            location,
            name,
        })
    }

    /// Attempts to infer a record access. If the attempt fails, then will fallback to attempting to infer a module access.
    /// If both fail, then the error from the record access will be used.
    fn infer_field_access(
        &mut self,
        container: UntypedExpr,

        // The SrcSpan of the entire field access:
        // ```gleam
        //    wibble.wobble
        // // ^^^^^^^^^^^^^ This
        // ```
        //
        location: SrcSpan,
        label: EcoString,

        // The SrcSpan of the selection label:
        // ```gleam
        //    wibble.wobble
        // // ^^^^^^ This
        // ```
        //
        label_location: SrcSpan,
        usage: FieldAccessUsage,
    ) -> TypedExpr {
        let container_location = container.location();

        // Computes a potential module access. This will be used if a record access can't be used.
        // Computes both the inferred access and if it shadows a variable.
        let module_access = if let UntypedExpr::Var { name, .. } = &container {
            let module_access =
                self.infer_module_access(name, label.clone(), &container_location, label_location);
            // Returns the result and if it shadows an existing variable in scope
            Some((module_access, self.environment.scope.contains_key(name)))
        } else {
            None
        };
        let record = if let UntypedExpr::Var { location, name } = container {
            // If the left-hand-side of the record access is a variable, this might actually be
            // module access. In that case, we only want to register a reference to the variable
            // if we actually referencing it in the record access.
            self.infer_var(
                name,
                location,
                ValueUsage::Other,
                ReferenceRegistration::DoNotRegister,
            )
        } else {
            self.infer_or_error(container)
        };
        // TODO: is this clone avoidable? we need to box the record for inference in both
        // the success case and in the valid record but invalid label case
        let record_access = match record.clone() {
            Ok(record) => self.infer_known_record_expression_access(
                record,
                label.clone(),
                location,
                label_location,
                container_location.start,
                usage,
            ),
            Err(e) => Err(e),
        };
        match (record_access, module_access) {
            // Record access is valid
            (Ok(record_access), _) => {
                if let TypedExpr::RecordAccess { record, label, .. } = &record_access {
                    // The programmer wrote `record.label`, so register a
                    // reference to the field for the language server.
                    if let Some((type_module, type_name)) = record.type_().named_type_name() {
                        self.environment.references.register_label_reference(
                            LabelOwner::Record {
                                module: type_module,
                                name: type_name,
                            },
                            label.clone(),
                            label_location,
                            LabelSyntax::Longhand,
                        );
                    }
                    // If this is actually record access and not module access,
                    // and we didn't register the variable reference earlier, we
                    // register it now.
                    if let TypedExpr::Var {
                        location,
                        constructor,
                        name,
                    } = record.as_ref()
                    {
                        self.register_value_constructor_reference(
                            name,
                            &constructor.variant,
                            *location,
                            ReferenceKind::Unqualified,
                        );
                    }
                }
                record_access
            }
            // Record access is invalid but module access is valid
            (
                _,
                Some((
                    Ok(TypedExpr::ModuleSelect {
                        location,
                        field_start,
                        type_,
                        label,
                        module_name,
                        module_alias,
                        constructor,
                    }),
                    _,
                )),
            ) => {
                // We only register the reference here, if we know that this is a module access.
                // Otherwise we would register module access even if we are actually accessing
                // the field on a record
                let module_location =
                    SrcSpan::new(location.start, location.start + module_alias.len() as u32);
                self.environment.references.register_value_reference(
                    module_name.clone(),
                    label.clone(),
                    &label,
                    SrcSpan::new(field_start, location.end),
                    ReferenceKind::Qualified {
                        module_alias: module_alias.clone(),
                        module_location,
                    },
                );
                TypedExpr::ModuleSelect {
                    location,
                    field_start,
                    type_,
                    label,
                    module_name,
                    module_alias,
                    constructor,
                }
            }
            // If module access failed because the module exists but that module does not export
            // the referenced value, we return extra information about the invalid module select,
            // so that we have information about the attempted module select and can use it, for
            // example, in the "Generate function" code action to support other modules.
            (
                _,
                Some((
                    Err(Error::UnknownModuleValue {
                        name,
                        module_name,
                        location,
                        value_constructors,
                        type_with_same_name,
                        context,
                    }),
                    false,
                )),
            ) => {
                self.problems.error(Error::UnknownModuleValue {
                    name,
                    module_name: module_name.clone(),
                    location,
                    value_constructors,
                    type_with_same_name,
                    context,
                });
                TypedExpr::Invalid {
                    location,
                    type_: self.new_unbound_var(),
                    extra_information: Some(InvalidExpression::ModuleSelect { module_name, label }),
                }
            }
            // If module access failed for some other reason, and no local variable shadows the
            // module, we just return an invalid expression.
            (_, Some((Err(module_access_err), false))) => {
                self.problems.error(module_access_err);
                TypedExpr::Invalid {
                    location,
                    type_: self.new_unbound_var(),
                    extra_information: None,
                }
            }
            // In any other case use the record access for the error
            (Err(record_access_err), _) => {
                self.problems.error(record_access_err);
                match record {
                    // If the record is valid then use a placeholder access
                    // This allows autocomplete to know a record access is being attempted
                    // Even if the access is not valid
                    Ok(record) => TypedExpr::RecordAccess {
                        location,
                        field_start: container_location.start,
                        type_: self.new_unbound_var(),
                        label: "".into(),
                        index: u64::MAX,
                        record: Box::new(record),
                        documentation: None,
                    },
                    Err(_) => TypedExpr::Invalid {
                        location,
                        type_: self.new_unbound_var(),
                        extra_information: None,
                    },
                }
            }
        }
    }

    fn infer_array_index(
        &mut self,
        tuple: UntypedExpr,
        index: u64,
        location: SrcSpan,
    ) -> Result<TypedExpr, Error> {
        if let UntypedExpr::TupleIndex { .. } = tuple {
            self.track_feature_usage(FeatureKind::NestedTupleAccess, location);
        }

        let tuple = self.infer_or_error(tuple)?;
        match collapse_links(tuple.type_()).as_ref() {
            Type::Tuple { elements } => {
                let type_ = elements
                    .get(index as usize)
                    .ok_or_else(|| Error::OutOfBoundsTupleIndex {
                        location: SrcSpan {
                            start: tuple.location().end,
                            end: location.end,
                        },
                        index,
                        size: elements.len(),
                    })?
                    .clone();
                Ok(TypedExpr::TupleIndex {
                    location,
                    index,
                    tuple: Box::new(tuple),
                    type_,
                })
            }

            type_ if type_.is_unbound() => Err(Error::NotATupleUnbound {
                location: tuple.location(),
            }),

            Type::Named { .. } | Type::Fn { .. } | Type::Var { .. } => Err(Error::NotATuple {
                location: tuple.location(),
                given: tuple.type_(),
            }),
        }
    }

    /// Same as `self.infer_or_error` but instead of returning a `Result` with an error,
    /// records the error and returns an invalid expression.
    pub fn infer(&mut self, expression: UntypedExpr) -> TypedExpr {
        let location = expression.location();
        match self.infer_or_error(expression) {
            Ok(result) => result,
            Err(error) => {
                let information = if let Error::UnknownVariable { name, .. } = &error {
                    Some(InvalidExpression::UnknownVariable { name: name.clone() })
                } else {
                    None
                };

                self.problems.error(error);
                self.error_expr_with_information(location, information)
            }
        }
    }

    /// Infers the type of the given function and tries to unify it with the
    /// given type, recording any unification error that might take place.
    /// The typed expression is returned in any case.
    ///
    pub fn infer_and_unify(&mut self, expression: UntypedExpr, type_: Arc<Type>) -> TypedExpr {
        let expression = self.infer(expression);
        if let Err(error) = unify(type_, expression.type_()) {
            self.problems
                .error(convert_unify_error(error, expression.location()));
        }
        expression
    }

    fn infer_binop(
        &mut self,
        operator: BinOp,
        operator_start: u32,
        left: UntypedExpr,
        right: UntypedExpr,
        location: SrcSpan,
    ) -> TypedExpr {
        let (input_type, output_type) = match &operator {
            BinOp::Eq | BinOp::NotEq => {
                let left = self.infer(left);
                let right = self.infer(right);
                if let Err(error) = unify(left.type_(), right.type_()) {
                    self.problems
                        .error(convert_unify_error(error, right.location()));
                } else {
                    // We only want to warn for redundant comparisons if it
                    // makes sense to compare the two values.
                    // That is, their types should match!
                    self.check_for_redundant_comparison(operator, &left, &right, location);
                }

                self.check_for_inefficient_empty_list_check(operator, &left, &right, location);

                return TypedExpr::BinOp {
                    location,
                    operator,
                    operator_start,
                    type_: bool(),
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
            BinOp::And => (bool(), bool()),
            BinOp::Or => (bool(), bool()),
            BinOp::LtInt => (int(), bool()),
            BinOp::LtEqInt => (int(), bool()),
            BinOp::LtFloat => (float(), bool()),
            BinOp::LtEqFloat => (float(), bool()),
            BinOp::GtEqInt => (int(), bool()),
            BinOp::GtInt => (int(), bool()),
            BinOp::GtEqFloat => (float(), bool()),
            BinOp::GtFloat => (float(), bool()),
            BinOp::AddInt => (int(), int()),
            BinOp::AddFloat => (float(), float()),
            BinOp::SubInt => (int(), int()),
            BinOp::SubFloat => (float(), float()),
            BinOp::MultInt => (int(), int()),
            BinOp::MultFloat => (float(), float()),
            BinOp::DivInt => (int(), int()),
            BinOp::DivFloat => (float(), float()),
            BinOp::RemainderInt => (int(), int()),
            BinOp::Concatenate => (string(), string()),
        };

        let left = self.infer(left);
        let right = self.infer(right);
        let unify_left = unify(input_type.clone(), left.type_());
        let unify_right = unify(input_type, right.type_());

        if unify_left.is_ok() && unify_right.is_ok() {
            // We only want to warn for redundant comparisons if it makes sense
            // to compare the two values. That is, their types should match!
            self.check_for_redundant_comparison(operator, &left, &right, location);
        }

        // There's some common cases in which we can provide nicer error messages:
        // - if we're using a float operator on int values
        // - if we're using an int operator on float values
        // - if we're using `+` on strings
        if operator.is_float_operator() && left.type_().is_int() && right.type_().is_int() {
            self.problems.error(Error::FloatOperatorOnInts {
                operator,
                location: SrcSpan::new(operator_start, operator_start + operator.size()),
            });
        } else if operator.is_int_operator() && left.type_().is_float() && right.type_().is_float()
        {
            self.problems.error(Error::IntOperatorOnFloats {
                operator,
                location: SrcSpan::new(operator_start, operator_start + operator.size()),
            });
        } else if operator == BinOp::AddInt && left.type_().is_string() && right.type_().is_string()
        {
            self.problems.error(Error::StringConcatenationWithAddInt {
                location: SrcSpan::new(operator_start, operator_start + operator.size()),
            });
        } else {
            // In all other cases we just report an error for each of the operands.
            if let Err(error) = unify_left {
                self.problems.error(
                    error
                        .operator_situation(operator)
                        .into_error(left.type_defining_location()),
                );
            }
            if let Err(error) = unify_right {
                self.problems.error(
                    error
                        .operator_situation(operator)
                        .into_error(right.type_defining_location()),
                );
            }
        }

        self.check_for_inefficient_empty_list_check(operator, &left, &right, location);

        TypedExpr::BinOp {
            location,
            operator,
            operator_start,
            type_: output_type,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn infer_assignment(&mut self, assignment: UntypedAssignment) -> TypedAssignment {
        let Assignment {
            pattern,
            value,
            kind,
            annotation,
            location,
            compiled_case: _,
        } = assignment;
        let value = self.expr_in_new_scope(|this| this.infer(value));
        let type_ = value.type_();
        let kind = self.infer_assignment_kind(kind);

        // Ensure the pattern matches the type of the value
        let mut pattern_typer = pattern::PatternTyper::new(
            self.environment,
            &self.current_function_definition,
            &self.hydrator,
            self.problems,
            PatternPosition::LetAssignment,
        );

        let pattern = pattern_typer.infer_single_pattern(pattern, &value);

        let minimum_required_version = pattern_typer.minimum_required_version;
        if minimum_required_version > self.minimum_required_version {
            self.minimum_required_version = minimum_required_version;
        }

        let pattern_typechecked_successfully = !pattern_typer.error_encountered;

        // Check that any type annotation is accurate.
        if let Some(annotation) = &annotation {
            match self
                .type_from_ast(annotation)
                .map(|type_| self.instantiate(type_, &mut hashmap![]))
            {
                Ok(annotated_type) => {
                    if let Err(error) = unify(annotated_type, type_)
                        .map_err(|error| convert_unify_error(error, value.type_defining_location()))
                    {
                        self.problems.error(error);
                    }
                }
                Err(error) => {
                    self.problems.error(error);
                }
            }
        }

        // The exhaustiveness checker expects patterns to be valid and to type check;
        // if they are invalid, it will crash. Therefore, if any errors were found
        // when type checking the pattern, we don't perform the exhaustiveness check.
        if !pattern_typechecked_successfully {
            return Assignment {
                location,
                annotation,
                kind,
                compiled_case: CompiledCase::failure(),
                pattern,
                value,
            };
        }

        let (output, not_exhaustive_error) =
            self.check_let_exhaustiveness(location, value.type_(), &pattern);

        match (&kind, not_exhaustive_error) {
            // The pattern is exhaustive in a let assignment, there's no problem here.
            (AssignmentKind::Let | AssignmentKind::Generated, Ok(_)) => (),

            // If the pattern is not exhaustive and we're not asserting we want to
            // report the error!
            (AssignmentKind::Let | AssignmentKind::Generated, Err(e)) => {
                self.problems.error(e);
            }

            // If we're asserting but the pattern already covers all cases then the
            // `assert` is redundant and can be safely removed.
            (
                AssignmentKind::Assert {
                    location,
                    assert_keyword_start,
                    ..
                },
                Ok(_),
            ) => self.problems.warning(Warning::RedundantAssertAssignment {
                location: SrcSpan::new(*assert_keyword_start, location.end),
            }),

            // Otherwise, if the pattern is never reachable (through variant inference),
            // we can warn the user about this.
            (AssignmentKind::Assert { .. }, Err(_)) => {
                // There is only one pattern to match, so it is index 0
                match output.is_reachable(0, 0) {
                    Reachability::Unreachable(UnreachablePatternReason::ImpossibleVariant) => self
                        .problems
                        .warning(Warning::AssertAssignmentOnImpossiblePattern {
                            location: pattern.location(),
                            reason: AssertImpossiblePattern::InferredVariant,
                        }),

                    Reachability::Unreachable(UnreachablePatternReason::ImpossibleSegments(
                        segments,
                    )) => self
                        .problems
                        .warning(Warning::AssertAssignmentOnImpossiblePattern {
                            location: pattern.location(),
                            reason: AssertImpossiblePattern::ImpossibleSegments { segments },
                        }),
                    // A duplicate pattern warning should not happen, since there is only one pattern.
                    Reachability::Reachable
                    | Reachability::Unreachable(UnreachablePatternReason::DuplicatePattern) => {}
                }
            }
        }

        Assignment {
            location,
            annotation,
            kind,
            compiled_case: output.compiled_case,
            pattern,
            value,
        }
    }

    fn infer_assignment_kind(
        &mut self,
        kind: AssignmentKind<UntypedExpr>,
    ) -> AssignmentKind<TypedExpr> {
        match kind {
            AssignmentKind::Let => AssignmentKind::Let,
            AssignmentKind::Generated => AssignmentKind::Generated,
            AssignmentKind::Assert {
                location,
                message,
                assert_keyword_start,
            } => {
                self.purity = Purity::Impure;
                let message = match message {
                    Some(message) => {
                        self.track_feature_usage(
                            FeatureKind::LetAssertWithMessage,
                            message.location(),
                        );
                        Some(self.infer_and_unify(message, string()))
                    }
                    None => None,
                };
                AssignmentKind::Assert {
                    location,
                    message,
                    assert_keyword_start,
                }
            }
        }
    }

    /// Returns a tuple with the typed clause and a bool that is true if an error
    /// was encountered while typing the clause patterns.
    ///
    fn infer_clause(
        &mut self,
        clause: UntypedClause,
        subjects: &[TypedExpr],
    ) -> (TypedClause, bool) {
        let Clause {
            pattern,
            alternative_patterns,
            guard,
            then,
            location,
        } = clause;
        self.value_in_new_scope(|this| {
            let (typed_pattern, typed_alternatives, error_encountered) =
                this.infer_clause_pattern(pattern, alternative_patterns, subjects, &location);

            let guard = this.infer_optional_clause_guard(guard);
            let then = this.infer(then);
            let clause = Clause {
                location,
                pattern: typed_pattern,
                alternative_patterns: typed_alternatives,
                guard,
                then,
            };
            (clause, error_encountered)
        })
    }

    fn infer_clause_pattern(
        &mut self,
        pattern: UntypedMultiPattern,
        alternatives: Vec<UntypedMultiPattern>,
        subjects: &[TypedExpr],
        location: &SrcSpan,
    ) -> (TypedMultiPattern, Vec<TypedMultiPattern>, bool) {
        let mut pattern_typer = pattern::PatternTyper::new(
            self.environment,
            &self.current_function_definition,
            &self.hydrator,
            self.problems,
            PatternPosition::CaseClause,
        );

        let typed_pattern = pattern_typer.infer_multi_pattern(pattern, subjects);

        // Each case clause has one or more patterns that may match the
        // subject in order for the clause to be selected, so we must type
        // check every pattern.
        let mut typed_alternatives = Vec::with_capacity(alternatives.len());
        for m in alternatives {
            typed_alternatives
                .push(pattern_typer.infer_alternative_multi_pattern(m, subjects, location));
        }

        let minimum_required_version = pattern_typer.minimum_required_version;
        if minimum_required_version > self.minimum_required_version {
            self.minimum_required_version = minimum_required_version;
        }

        (
            typed_pattern,
            typed_alternatives,
            pattern_typer.error_encountered,
        )
    }

    fn infer_optional_clause_guard(
        &mut self,
        guard: Option<UntypedClauseGuard>,
    ) -> Option<TypedClauseGuard> {
        // If there is a guard we type check it and assert that it is of type
        // Bool.
        let guard = GuardTyper::new(self).infer(guard?);
        if let Err(error) = unify(bool(), guard.type_()) {
            self.problems
                .error(convert_unify_error(error, guard.location()));
        }
        Some(guard)
    }
        */
}