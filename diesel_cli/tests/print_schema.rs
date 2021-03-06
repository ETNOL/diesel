use support::{database, project};

#[test]
fn run_infer_schema() {
    let p = project("print_schema").build();
    let db = database(&p.database_url());

    p.command("setup").run();

    db.execute("CREATE TABLE users1 (id INTEGER PRIMARY KEY);");
    db.execute("CREATE TABLE users2 (id INTEGER PRIMARY KEY);");

    assert!(db.table_exists("users1"));
    assert!(db.table_exists("users2"));

    let result = p.command("print-schema").run();

    assert!(result.is_success(), "Result was unsuccessful {:?}", result);
    if cfg!(feature = "sqlite") {
        assert_eq!(result.stdout(),
r"table! {
    /// Representation of the `users1` table.
    ///
    /// (Automatically generated by Diesel.)
    users1 (id) {
        /// The `id` column of the `users1` table.
        ///
        /// Its SQL type is `Nullable<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Nullable<Integer>,
    }
}

table! {
    /// Representation of the `users2` table.
    ///
    /// (Automatically generated by Diesel.)
    users2 (id) {
        /// The `id` column of the `users2` table.
        ///
        /// Its SQL type is `Nullable<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Nullable<Integer>,
    }
}
");
    } else if cfg!(feature = "postgres") {
                assert_eq!(result.stdout(),
r"table! {
    /// Representation of the `users1` table.
    ///
    /// (Automatically generated by Diesel.)
    users1 (id) {
        /// The `id` column of the `users1` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
    }
}

table! {
    /// Representation of the `users2` table.
    ///
    /// (Automatically generated by Diesel.)
    users2 (id) {
        /// The `id` column of the `users2` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
    }
}
");
    }
}

#[test]
fn run_infer_schema_whitelist() {
    let p = project("print_schema_whitelist").build();
    let db = database(&p.database_url());

    p.command("setup").run();

    db.execute("CREATE TABLE users1 (id INTEGER PRIMARY KEY);");
    db.execute("CREATE TABLE users2 (id INTEGER PRIMARY KEY);");

    assert!(db.table_exists("users1"));
    assert!(db.table_exists("users2"));

    let result = p.command("print-schema").arg("users1").arg("-w").run();

    assert!(result.is_success(), "Result was unsuccessful {:?}", result);
    if cfg!(feature = "sqlite") {
        assert_eq!(result.stdout(),
r"table! {
    /// Representation of the `users1` table.
    ///
    /// (Automatically generated by Diesel.)
    users1 (id) {
        /// The `id` column of the `users1` table.
        ///
        /// Its SQL type is `Nullable<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Nullable<Integer>,
    }
}
");
    } else if cfg!(feature = "postgres") {
        assert_eq!(result.stdout(),
r"table! {
    /// Representation of the `users1` table.
    ///
    /// (Automatically generated by Diesel.)
    users1 (id) {
        /// The `id` column of the `users1` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
    }
}
");
    }
}

#[test]
fn run_infer_schema_blacklist() {
    let p = project("print_schema_blacklist").build();
    let db = database(&p.database_url());

    p.command("setup").run();

    db.execute("CREATE TABLE users1 (id INTEGER PRIMARY KEY);");
    db.execute("CREATE TABLE users2 (id INTEGER PRIMARY KEY);");

    assert!(db.table_exists("users1"));
    assert!(db.table_exists("users2"));

    let result = p.command("print-schema").arg("users1").arg("-b").run();

    assert!(result.is_success(), "Result was unsuccessful {:?}", result);
    if cfg!(feature = "sqlite") {
        assert_eq!(result.stdout(),
r"table! {
    /// Representation of the `users2` table.
    ///
    /// (Automatically generated by Diesel.)
    users2 (id) {
        /// The `id` column of the `users2` table.
        ///
        /// Its SQL type is `Nullable<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Nullable<Integer>,
    }
}
");
    } else if cfg!(feature = "postgres") {
        assert_eq!(result.stdout(),
r"table! {
    /// Representation of the `users2` table.
    ///
    /// (Automatically generated by Diesel.)
    users2 (id) {
        /// The `id` column of the `users2` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
    }
}
");
    }
}

#[test]
fn run_infer_schema_order() {
    let p = project("print_schema_order").build();
    let db = database(&p.database_url());

    p.command("setup").run();

    db.execute("CREATE TABLE def (id INTEGER PRIMARY KEY);");
    db.execute("CREATE TABLE abc (id INTEGER PRIMARY KEY);");
    db.execute("CREATE TABLE ghi (id INTEGER PRIMARY KEY);");

    assert!(db.table_exists("def"));
    assert!(db.table_exists("abc"));
    assert!(db.table_exists("ghi"));

    let result = p.command("print-schema").run();

    assert!(result.is_success(), "Result was unsuccessful {:?}", result);
    if cfg!(feature = "sqlite") {
        assert_eq!(result.stdout(),
r"table! {
    /// Representation of the `abc` table.
    ///
    /// (Automatically generated by Diesel.)
    abc (id) {
        /// The `id` column of the `abc` table.
        ///
        /// Its SQL type is `Nullable<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Nullable<Integer>,
    }
}

table! {
    /// Representation of the `def` table.
    ///
    /// (Automatically generated by Diesel.)
    def (id) {
        /// The `id` column of the `def` table.
        ///
        /// Its SQL type is `Nullable<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Nullable<Integer>,
    }
}

table! {
    /// Representation of the `ghi` table.
    ///
    /// (Automatically generated by Diesel.)
    ghi (id) {
        /// The `id` column of the `ghi` table.
        ///
        /// Its SQL type is `Nullable<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Nullable<Integer>,
    }
}
");
    } else if cfg!(feature = "postgres") {
        assert_eq!(result.stdout(),
r"table! {
    /// Representation of the `abc` table.
    ///
    /// (Automatically generated by Diesel.)
    abc (id) {
        /// The `id` column of the `abc` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
    }
}

table! {
    /// Representation of the `def` table.
    ///
    /// (Automatically generated by Diesel.)
    def (id) {
        /// The `id` column of the `def` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
    }
}

table! {
    /// Representation of the `ghi` table.
    ///
    /// (Automatically generated by Diesel.)
    ghi (id) {
        /// The `id` column of the `ghi` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
    }
}
");
    }
}

#[test]
fn run_infer_schema_compound_primary_key() {
    let p = project("print_schema_compound_primary_key").build();
    let db = database(&p.database_url());

    p.command("setup").run();

    db.execute("CREATE TABLE asd (id INTEGER, qsd INTEGER, PRIMARY KEY (id, qsd));");

    let result = p.command("print-schema").run();

    assert!(result.is_success(), "Result was unsuccessful {:?}", result);

    if cfg!(feature = "sqlite") {
        assert_eq!(result.stdout(),
r"table! {
    /// Representation of the `asd` table.
    ///
    /// (Automatically generated by Diesel.)
    asd (id, qsd) {
        /// The `id` column of the `asd` table.
        ///
        /// Its SQL type is `Nullable<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Nullable<Integer>,
        /// The `qsd` column of the `asd` table.
        ///
        /// Its SQL type is `Nullable<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        qsd -> Nullable<Integer>,
    }
}
");
    } else if cfg!(feature = "postgtres") {
        assert_eq!(result.stdout(),
r"table! {
    /// Representation of the `asd` table.
    ///
    /// (Automatically generated by Diesel.)
    asd (id, qsd) {
        /// The `id` column of the `asd` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `iqsd` column of the `asd` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        qsd -> Int4,
    }
}
");
    }
}

#[test]
#[cfg(feature = "postgres")]
fn print_schema_specifying_schema_name() {
    let p = project("print_schema_specifying_schema_name").build();
    let db = database(&p.database_url());

    p.command("setup").run();

    db.execute("CREATE SCHEMA custom_schema");
    db.execute("CREATE TABLE in_public (id SERIAL PRIMARY KEY)");
    db.execute("CREATE TABLE custom_schema.in_schema (id SERIAL PRIMARY KEY)");

    let result = p.command("print-schema").arg("--schema").arg("custom_schema").run();

    assert!(result.is_success(), "Result was unsuccessful {:?}", result);

    assert_eq!(result.stdout(),
r"pub mod custom_schema {
    table! {
        /// Representation of the `custom_schema.in_schema` table.
        ///
        /// (Automatically generated by Diesel.)
        custom_schema.in_schema (id) {
            /// The `id` column of the `custom_schema.in_schema` table.
            ///
            /// Its SQL type is `Int4`.
            ///
            /// (Automatically generated by Diesel.)
            id -> Int4,
        }
    }
}
");
}

#[test]
#[cfg(feature = "postgres")]
fn print_schema_with_foreign_keys() {
    let p = project("print_schema_with_foreign_keys").build();
    let db = database(&p.database_url());

    p.command("setup").run();

    db.execute("CREATE TABLE users (id SERIAL PRIMARY KEY)");
    db.execute("CREATE TABLE posts (id SERIAL PRIMARY KEY, user_id INTEGER NOT NULL REFERENCES users)");
    db.execute("CREATE TABLE comments (id SERIAL PRIMARY KEY, post_id INTEGER NOT NULL REFERENCES posts)");

    let result = p.command("print-schema").run();

    assert!(result.is_success(), "Result was unsuccessful {:?}", result);

    assert_eq!(result.stdout(),
r"table! {
    /// Representation of the `comments` table.
    ///
    /// (Automatically generated by Diesel.)
    comments (id) {
        /// The `id` column of the `comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `post_id` column of the `comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        post_id -> Int4,
    }
}

table! {
    /// Representation of the `posts` table.
    ///
    /// (Automatically generated by Diesel.)
    posts (id) {
        /// The `id` column of the `posts` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `user_id` column of the `posts` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
    }
}

table! {
    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
    }
}

joinable!(posts -> users (user_id));
joinable!(comments -> posts (post_id));
");
}
