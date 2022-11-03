table! {
    phone_report (id) {
        id -> Int4,
        phone_number -> Varchar,
        origin_name -> Text,
        count_confirm -> Int4,
        count_report -> Int4,
        orgin_image ->  Text,
        confirmed -> Bool,
        change_owner -> Bool,
        is_deleted -> Bool,
        date_created -> Varchar,
        date_modified -> Varchar,
    }
}

table! {
    phone_search (id) {
        id -> Int4,
        phone_number -> Varchar,
        count_search -> Int4,
        date_created -> Varchar,
        date_modified -> Varchar,
    }
}

table! {
    phone_comment (id) {
        id -> Int4,
        phone_id -> Int4,
        name_user -> Varchar,
        body_comment -> Text,
        date_created -> Varchar,
        date_modified -> Varchar,
    }
}


// table! {
//     app_report (id) {
//         id -> Int4,
//         app_name -> Varchar,
//         app_image -> Varchar,
//         app_bundle_android -> Varchar,
//         app_bundle_ios -> Varchar,
//         app_des -> Text,
//         count_report -> Int4,
//         category_type -> Int4,
//         date_created -> Varchar,
//         date_modified -> Varchar,
//     }
// }

// table! {
//     app_comment (id) {
//         id -> Int4,
//         app_id -> Int4,
//         name_user -> Varchar,
//         body_comment -> Text,
//         date_created -> Varchar,
//         date_modified -> Varchar,
//     }
// }

// table! {
//     link_report (id) {
//         id -> Int4,
//         link_des -> Varchar,
//         origin_name -> Text,
//         count_confirm -> Int4,
//         count_report -> Int4,
//         category_id - Int4,
//         date_created -> Varchar,
//         date_modified -> Varchar,
//     }
// }

// table! {
//     link_category (id) {
//         id -> Int4,
//         type_link -> Varchar,
//         image_link -> Text,
//         date_created -> Varchar,
//         date_modified -> Varchar,
//     }
// }

// table! {
//     link_search (id) {
//         id -> Int4,
//         link_des -> Varchar,
//         count_search -> Int4,
//         date_created -> Varchar,
//         date_modified -> Varchar,
//     }
// }

// table! {
//     link_comment (id) {
//         id -> Int4,
//         link_id -> Int4,
//         name_user -> Varchar,
//         body_comment -> Text,
//         date_created -> Varchar,
//         date_modified -> Varchar,
//     }
// }