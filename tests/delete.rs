/*
 * Copyright (c) 2017-2018 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#![feature(proc_macro)]

extern crate tql;
#[macro_use]
extern crate tql_macros;

#[macro_use]
mod connection;

backend_extern_crate!();

use tql::PrimaryKey;
use tql_macros::to_sql;

#[derive(SqlTable)]
#[allow(dead_code)]
struct Table {
    id: PrimaryKey,
    field1: String,
    field2: i32,
}

#[test]
fn test_delete() {
    //assert_eq!(
        //"DELETE FROM Table",
        //to_sql!(Table.delete()) // TODO: this does not work because the errors (including
        //warnings) return a dummy result.
    //);
    assert_eq!(
        "DELETE FROM Table WHERE Table.field1 = 'test'",
        to_sql!(Table.filter(field1 == "test").delete())
    );
    assert_eq!(
        "DELETE FROM Table WHERE Table.id = $1",
        to_sql!(Table.get(id).delete())
    );
}
