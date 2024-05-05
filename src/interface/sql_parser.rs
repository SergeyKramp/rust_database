use nom::*;

pub fn parse_query(input: &str) -> IResult<&str, Query> {
    let (input, _) = tag("SELECT")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, column_names) = parse_column_names(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("FROM")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, table_name) = parse_table_name(input)?;
    let (input, _) = multispace0(input)?;
    let (input, conditions) = parse_conditions(input)?;
    Ok((
        input,
        Query {
            column_names,
            table_name,
            conditions,
        },
    ))
}

fn parse_column_names(input: &str) -> IResult<&str, Vec<String>> {
    let (input, column_names) = separated_list0(tag(","), parse_column_name)(input)?;
    Ok((input, column_names))
}

fn parse_table_name(input: &str) -> IResult<&str, String> {
    let (input, table_name) = parse_column_name(input)?;
    Ok((input, table_name))
}

fn parse_conditions(input: &str) -> IResult<&str, Vec<Condition>> {
    let (input, _) = tag("WHERE")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, conditions) = separated_list0(tag("AND"), parse_condition)(input)?;
    Ok((input, conditions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_query_test() {
        let input = "SELECT name, age FROM people WHERE age > 21 AND name = 'Alice'";
        let result = parse_query(input);
        assert_eq!(
            result,
            Ok((
                "",
                Query {
                    column_names: vec!["name".to_string(), "age".to_string()],
                    table_name: "people".to_string(),
                    conditions: vec![
                        Condition {
                            column_name: "age".to_string(),
                            operator: Operator::GreaterThan,
                            value: Value::Int(21),
                        },
                        Condition {
                            column_name: "name".to_string(),
                            operator: Operator::Equal,
                            value: Value::String("Alice".to_string()),
                        },
                    ],
                }
            ))
        );
    }
}
