fn bubble_sort(v:&mut Vec<i32>){
    let mut n=v.len();
    while n>0{
        let (mut i,mut ptr_max)=(1,0);
        while i<n{
           if v[i-1]>v[i]{
                v.swap(i-1,i); 
                ptr_max=i;
           } 
           i+=1;
        }
        n=ptr_max;
    }
}

fn main(){
    let mut v=vec![1,3,2,9,12,5,4,7];
    bubble_sort(&mut v);
    println!("v :{:?}",v);
}
