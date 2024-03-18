use crate::expression::{
    is_aggregate, Expression, IsContainedInGroupBy, MixedAggregates, ValidGrouping,
};
use crate::sql_types::Date;
use crate::Column;

#[derive(Debug, Clone)]
pub struct DateExpression<T> {
    pub column: T,
}

impl<T> Expression for DateExpression<T>
where
    T: Expression,
{
    type SqlType = Date;
}

impl<T, GroupByClause> ValidGrouping<GroupByClause> for DateExpression<T>
where
    T: ValidGrouping<GroupByClause>,
    T::IsAggregate: MixedAggregates<is_aggregate::No>,
{
    type IsAggregate = <T::IsAggregate as MixedAggregates<is_aggregate::No>>::Output;
}

impl<T, Col> IsContainedInGroupBy<Col> for DateExpression<T>
where
    T: IsContainedInGroupBy<Col>,
    Col: Column,
{
    type Output = <T as IsContainedInGroupBy<Col>>::Output;
}
