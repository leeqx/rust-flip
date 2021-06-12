// ** 基础语法
fn meta_syntax() {
    // ** 注解,例如：#[derive(Debug)]
    // ** 宏例如：println! ,vec!
    // ** 引用 & 解引用 *  与c里面的取地址、访问地址内容类似

}
fn all_builtin_data_type() {
    // ** 所有内建的数据类型
    let ai8: i8 = -8;
    let au8: u8 = 8;
    let ai16: i16 = -16 ;
    let au16: u16 = 16;
    let ai32: i32 = -32;
    let au32: u32 = 32;
    let ai64: i64 = -64;
    let au64: u64 = 64;
    let ai128: i128 = -128;
    let au128: u128 = 128;

    let size: isize = -1;
    let size2:usize = 0;

    let float: f32 = 0.01;
    let double: f64 = 0.001;

   // ** 特殊表示格式 
    let var: u32 = 98_222; // equals 98222
    let hex: u32 = 0xff;
    let octal: u32 = 0o77; // 第二个字符是字母o
    let binary: u32 = 0b1111_0000; // one byte format as 8bit
    let byte: u8 = b'A';
    println!("{},{},{},{},{}", var, hex,octal,binary, byte);


    // ** tuple: 各个元素类型可以不同
    let tup:(i32,f32,u8) = (600,6.000,1);
    let (x,y,z) = tup;
    println!("this is tuple :{:#?},({},{},{}),({},{},{})", tup,
             x,y,z,//  两种访问方式
             tup.0, tup.1,tup.2); //  两种访问方式

    // ** array
    // *** 固定长度
    let array = [1,3,4,5]; 
    // *** 指定类型，并指定长度
    let array2: [i32;5] = [1,4,4,5,6];
    // *** 指定初始值为3，有5个元素
    let array3  = [3;5];
    println!("this is array:{:#?}\n{:#?}\n{:#?}",array,array2,array3);

    // ** vector 可变长度
    let mut vec1: Vec<u32> = Vec::new(); // 带有mut 表示该vec1 是可以变的可以往里面插入元素
    vec1.push(1);
    vec1.push(2);
    // *** 使用宏，并初始化三个元素
    let vec2 = vec![1,2,3]; // 这个是借用，不能更改，如果需要更改请加mut 修饰符
    //vec2.push(56); //错误
    let mut mut_vec3 = vec![2,3,4];
    mut_vec3.push(100);

    println!("this is vector:{:#?}\n{:#?}\n{:#?}", vec1,vec2,mut_vec3);


    // ** 遍历数组
    {
        let mut vec5 = vec![1,21,33];
        for i in &mut vec5 { //修改vector的元素，vec5 也必须是mutable
           *i += 100; 
        }
        for i in vec5 {
            println!("{}",i);
        }

    }


    // ** 所有权
    { // 所有权
        // vector 元素访问方式有三种：& [] 返回的都是一个引用
        let vec4 = vec![1,3,4,5];
        let vec_ref = &vec4[0];
        //*vec_ref = 123; // 因为 =vec_ref=是不能修改的,引用的是4
        let mut mut_vec_ref = &vec4[0];
        //*mut_vec_ref = 123; // 即使是mut 也不能够修改

        // 下面这种方式是可以修改的
        let mut mut_vec4 = vec![1,2,3] ;
        let mut_vec_ref2 = &mut mut_vec4[0];
        *mut_vec_ref2 = 123;


        //let value_panic = vec4[4];// out of range 会引发panic
        println!("get element from vector:{},with &:{}", vec4[0],&vec4[0]);

        // 如果为了避免访问下面不存在的，所以我们建议通过get来获取元素
        match vec4.get(3) { // match 使用请参考： [[控制分支]]
            Some(value) => { // 表示4 索引存在
                println!("get element from vector:{}",value);
            },
            None => println!("index out of range"),
        }
    } // vec4 离开这个作用域就会被释放，因为他的所有权范围在该区域内


    // ** 枚举类型
    #[derive(Debug)] // 调试输出注解
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3) ,
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        println!("{:#?}",i);
    }


    // ** <TOTO> 字符串
}

//* 控制分支 #<<控制分支>>
fn control() {
    
}

fn main() {
    println!("------------------------");
    println!("\n\nwelcome to rust.....");
    all_builtin_data_type();
}
