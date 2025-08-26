fn main() {
    let a = [1,2,3,4,5];
    let mut arr = [0;5];// 초기화 안함

    for i in 0..5 {
        arr[i] +=1;
      
    }

    for num in arr.iter_mut(){
         *num += 1;
    }

    for i in arr.iter() {
        print!("{i} ");
    }
}
