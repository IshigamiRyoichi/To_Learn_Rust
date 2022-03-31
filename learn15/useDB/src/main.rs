use rusqlite::{params, Connection, Result};
use chrono::prelude::*;
use diesel::prelude::*;
use crate::diesel::Connection as OtherConnection;

fn test1() -> Result<()>{
    // インメモリーデータベースの作成
    let cn = Connection::open_in_memory()?;
    // テーブルの作成
    cn.execute("create table users (id integer, name text, age integer)", params![])?;
    cn.execute("insert into users (id, name, age) values(1, 'Kongo', 20)", params![])?;
    cn.execute("insert into users (id, name, age) values(2, 'Hieai', 20)", params![])?;
    cn.execute("insert into users (id, name, age) values(3, 'Haruna', 20)", params![])?;
    cn.execute("insert into users (id, name, age) values(4, 'Kirihara', 20)", params![])?;
    Ok(())
}

fn test2() -> Result<()>{
    // インメモリーデータベースの作成
    let cn = Connection::open_in_memory()?;
    cn.execute_batch(
        "
        create table users (id integer, name text, age integer);
        insert into users (id, name, age) values(1, 'Kongo', 20);
        insert into users (id, name, age) values(2, 'Hieai', 20);
        insert into users (id, name, age) values(3, 'Haruna', 20);
        insert into users (id, name, age) values(4, 'Kirihara', 20);
        "
    )?;
    Ok(())
}

fn test3() -> Result<()>{
    // インメモリーデータベースの作成
    let cn = Connection::open_in_memory()?;
    cn.execute("create table users (id integer, name text, age integer)", params![])?;
    let mut stmt = cn.prepare("insert into users (id, name, age) values(?,?,?)")?;
    stmt.execute(params![1, "Kongo", 20])?;
    stmt.execute(params![2, "Hieai", 20])?;
    stmt.execute(params![3, "Haruna", 18])?;
    stmt.execute(params![4, "Kirishima", 15])?;
    // クエリの作成 
    let mut stmt = cn.prepare("select * from users")?;
    let mut rows = stmt.query(params![])?;
    while let Some(row) = rows.next()?{
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let age: i32 = row.get(2)?;
        println!("id: {}, name: {}, age: {}",id, name, age);
    }
    Ok(())
}

#[derive(Debug)]
struct User{
    id: i32,
    name: String,
    age: i32,
}

fn test4() -> Result<()>{
    let cn = Connection::open_in_memory()?;
    // テーブルの作成
    cn.execute("create table users (id integer, name text, age integer)", params![])?;
    // データ入力
    let mut stmt = cn.prepare("insert into users (id, name, age) values(?,?,?)")?;
    stmt.execute(params![1, "Kongo", 20])?;
    stmt.execute(params![2, "Hieai", 20])?;
    stmt.execute(params![3, "Haruna", 18])?;
    stmt.execute(params![4, "Kirishima", 15])?;
    // クエリの作成 
    let mut stmt = cn.prepare("select * from users where")?;
    let iter = stmt.query_map(params![], |row| {
        Ok(User{
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;
    for it in iter{
        println!("{:?}", it.unwrap());
    }
    Ok(())
}

fn test5() -> Result<()>{
    let cn = Connection::open_in_memory()?;
    // テーブルの作成
    cn.execute("create table users (id integer, name text, age integer)", params![])?;
    // データ挿入
    let mut stmt = cn.prepare("insert into users (id, name, age) values(?,?,?)")?;
    stmt.execute(params![1, "Kongo", 20])?;
    stmt.execute(params![2, "Hieai", 20])?;
    stmt.execute(params![3, "Haruna", 18])?;
    stmt.execute(params![4, "Kirishima", 15])?;
    // クエリの作成 
    let mut stmt = cn.prepare("select * from users where age > ")?;
    let iter = stmt.query_map(params![15], |row| {
        Ok(User{
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;
    for it in iter{
        println!("{:?}", it.unwrap());
    }
    Ok(())

}

fn test6() -> Result<()>{
    let args = std::env::args().collect::<Vec<String>>();
    let cn = Connection::open("sample.db")?;
    if args.len() <= 1{
        let mut stmt = cn.prepare("select * from users")?;
        for it in stmt.query_map(params![], |row|{
            Ok(User{
                id: row.get(0)?,
                name: row.get(1)?,
                age: row.get(2)?,
            })
        })?{
            println!("{:?}",it.unwrap());
        }
    } else {
        match args[1].as_str() {
            "init" => {
                cn.execute("create table users (id integer, name text, age integer)", params![])?;
                println!("create databases");
            },
            "into" => {
                let id: i32 = args[2].parse::<i32>().unwrap();
                let name = &args[3];
                let age:i32 = args[4].parse::<i32>().unwrap();
                let mut stmt = cn.prepare("insert into users (id, name, age) values(?,?,?)")?;
                stmt.execute(params![id,name,age])?;
                println!("insert data");
            },
            _ => {
                println!("parameter error");
            },
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Fish{
    id: i32,
    date: NaiveDate,
    num: i32,
    name: String,
    sale: String,
    market: String,
}

// データベース結合
fn test7() -> Result<()>{
    // データベースへ接続
    let cn = Connection::open("sample.db")?;
    // クエリの作成
    let mut stmt = cn.prepare("
        SELECT
            t.id,
            t.日付,
            t.御売,
            T品名.name,
            T販売方法.name,
            T市場.name
        FROM T御売 t
        inner join T品名 on t.品名id = T品名id
        inner join T販売方法 on t.販売方法id = T販売方法.id
        inner join T市場 on t.市場id = T市場.id
        where t.日付 = '2020-01-12'
    ")?;
    let iter = stmt.query_map(params![], |row|{
        let dt: String = row.get(1)?;
        Ok(Fish{
            id: row.get(0)?,
            date: NaiveDate::parse_from_str(&dt, "%Y-%m-%d").unwrap(),
            num: row.get(2)?,
            name: row.get(3)?,
            sale: row.get(4)?,
            market: row.get(5)?,
        })
    })?;
    for it in iter{
        println!("{:?}", it.unwrap());
    }
    Ok(())
}

// データベースソート
fn test8() -> Result<()>{
    // データベースへ接続
    let cn = Connection::open("sample.db")?;
    // クエリの作成
    let mut stmt = cn.prepare("
        SELECT
            t.id,
            t.日付,
            t.御売,
            T品名.name,
            T販売方法.name,
            T市場.name
        FROM T御売 t
        inner join T品名 on t.品名id = T品名id
        inner join T販売方法 on t.販売方法id = T販売方法.id
        inner join T市場 on t.市場id = T市場.id
        where t.日付 = '2020-01-12'
            and T市場.name = '築地'
        order by t.御売数 desc
    ")?;
    let mut rows = stmt.query(params![])?;
    while let Some(row) = rows.next()?{
        let name: String = row.get(0)?;
        let sale: String = row.get(1)?;
        let sum: i32 = row.get(2)?;
        println!("{} {} 御売数:{}",name ,sale,sum);
    }
    Ok(())
}

#[macro_use]
extern crate diesel;
#[derive(Debug, Queryable)]
struct User{
    id: i32,
    name: String,
    age: i32,
}
table!{
    users (id){
        id -> Integer,
        name -> Text,
        age -> Integer,
    }
}

use self::users::dsl::*;

fn test9(){
    let url = "sample.db";
    let cn = SqliteConnection::establish(&url).expect("error: cannot connecting.");
    let result = users.filter(users::age.gt(15)).load::<User>(&cn).unwrap();
    println!("result.len is {}", result.len());
    for it in result{
        println!("id: {}, name: {}, age: {}",it.id, it.name, it.age);
    }
}

fn main() {
    test9();
}