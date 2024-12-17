use std::sync::Arc;
use deltalake::datafusion::execution::context::SessionContext;
use deltalake::arrow::util::pretty::print_batches;

#[tokio::main]
async fn main() {
    println!("Loading deltatbl-partitioned");
    // Mở table deltatbl-partitioned
    let table = deltalake::open_table("../data/deltatbl-partitioned").await.expect("Failed to open table");
    // In ra thông tin table
    println!("..loaded version {}", table.version());
    // In ra các file của table
    for file in table.get_files_iter() {
        println!(" - {}", file.as_ref());
    }

    // Tạo SessionContext chứa các bảng và thực thi các câu lệnh SQL
    let ctx = SessionContext::new();
    // Đăng ký bảng demo với dữ liệu từ table
    let table = deltalake::open_table("../data/deltatbl-partitioned").await.unwrap();
    // Đăng ký bảng demo với dữ liệu từ table
    ctx.register_table("demo", Arc::new(table)).unwrap();
    // Thực thi câu lệnh SQL
    let batches = ctx.sql("SELECT * FROM demo LIMIT 5").await.expect("Failed to execute SQL").collect().await.unwrap();
    print_batches(&batches).expect("Failed to print batches");

    // Thực thi câu lệnh SQL: SELECT * FROM demo WHERE c2 = 'foo0' để lấy ra dữ liệu cột c2 có giá trị là foo0
    let df = ctx.sql("SELECT * FROM demo WHERE c2 = 'foo0'").await.expect("Failed to create data frame");
    print_batches(&df.collect().await.expect("Failed to collect data")).expect("Failed to print batches");
}