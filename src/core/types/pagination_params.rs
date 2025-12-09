use serde::Serialize;

/// The parameters used to control pagination for a given paginated endpoint.
#[derive(Debug, Clone, Serialize, Default)]
pub struct PaginationParams<'a> {
    /// Upper limit on the number of objects to return, between 1 and 100. The default value is 10.
    pub limit: Option<u64>,

    /// Order the results by the creation time. Supported values are "asc" and "desc" for showing
    /// older and newer records first respectively. Default order is descending.
    pub order: Option<&'a PaginationOrder>,

    /// An object ID that defines your place in the list. When the ID is not present,
    /// you are at the end of the list.
    ///
    /// For example, if you make a list request and receive 100 objects, ending with "obj_123",
    /// your subsequent call can include after="obj_123" to fetch a new batch of objects
    /// after "obj_123".
    pub after: Option<&'a str>,

    /// An object ID that defines your place in the list. When the ID is not present, you are at the end of the list.
    ///
    /// For example, if you make a list request and receive 100 objects, ending with "obj_123",
    /// your subsequent call can include before="obj_123" to fetch a new batch of objects
    /// before "obj_123".
    pub before: Option<&'a str>,
}

/// The order in which records should be returned when paginating.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaginationOrder {
    /// Records are returned in ascending order.
    Asc,

    /// Records are returned in descending order.
    Desc,
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::PaginationOrder;

    #[test]
    fn pagination_order_properly_serializes_asc() {
        assert_eq!(
            serde_json::to_string(&PaginationOrder::Asc).unwrap(),
            json!("asc").to_string()
        )
    }

    #[test]
    fn pagination_order_properly_serializes_desc() {
        assert_eq!(
            serde_json::to_string(&PaginationOrder::Desc).unwrap(),
            json!("desc").to_string()
        )
    }
}
