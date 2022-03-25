use rusqlite::{params, Connection, Result};
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

fn main() {
    test6();
}