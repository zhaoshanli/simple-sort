//冒泡排序：使用泛型
fn pop_max<T :PartialOrd + Copy >( array: &mut [T ] )  {
    let mut temp : T  ;
    for i in 1 .. array.len() {
        if array[ 0 ] < array[ i ] {
            temp = array[ 0 ];
            array[ 0 ] = array[ i ];
            array[ i ] = temp;
        }
    }
}


fn main() {
    //整数数组的冒泡排序
    let mut arr = [ 1 , 21 , 2099 , 9 , 10 , 100 , 2 , 45 , 300 , 203 ];

    let size = arr.len();

    for i in 0 .. size - 1 {
        pop_max( &mut arr[ i.. size ] );
    }
    

    println!("Result -- \n ",  );
    for i in arr {
        println!(" {}", i );
    }

    println!(" >>>>>>>>Integer sort END\n\n" );

    //换成浮点数组
    let mut arr2 = [ 1.0 , 21.0 , 2099.1 , 9.4 , 10.2 , 100.1 , 2.0 , 45.3 , 300.1 , 203.0 ];

    let size = arr2.len();

    for i in 0 .. size - 1 {
        pop_max( &mut arr2[ i.. size ] );
    }

    println!("Result -- \n ",  );
    for i in arr {
        println!(" {}", i );
    }
    
    println!(" >>>>>>>>Float sort END" );

}