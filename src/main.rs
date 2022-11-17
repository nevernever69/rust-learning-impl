//author: never
//my implemention of  queue in rust
pub struct Queue{                   //create a struct with a  name Queue
    r : i32,
    f : i32,
    size : i32,
    arr:    Vec<i32>                

}

fn isempty(q: &mut Queue)->bool     //function that shows is queue empty 
{   
    if q.r==q.f                     //if both the pointer(index position) are at the same place
    {
        return true;                //then queue is empty
    
    }
    return false;                   //otherwise false
}   
fn isFull(q: &mut Queue)->bool      //is queue is full
{
    if q.r == q.size-1              
    {
        return true;                
    
    }
    return false;


}

fn enqueue(q: &mut Queue, val: i32)  
{       
    if isFull(q) == true
    {   
        println!("queue is full !!");
    }
    else                                //if queue is not full
    {
        q.r = q.r+1;                    //then increase the pointer
        q.arr.push(val);                //and push the value


    }
    
    

}

fn dequeue(q: &mut Queue )->i32
{
    let mut a = -1;
    if isempty(q) == true
    {
        println!("queue is empty");
        return -1;
    }
    else{
            q.f = q.f+1;                
            a = q.arr[q.f as usize];

    return a;
    }

}



fn main()
{
    let mut q: Queue  = Queue{
        r: -1,
        f: -1,
        size: 100,
        arr: Vec::new()
        
    };      

   /* let mut val =13; 
    enqueue(&mut q, val);
    val = 14;
   enqueue(&mut q, val);
    val = 15;
    enqueue(&mut q, val);
    let ans =isempty(&mut q);
    let ans1 = isFull(&mut q);
    println!("is queue empty {}", ans);
    println!("is queue Full {}", ans1);
    let ele = dequeue(&mut q);
    let ele1 = dequeue(&mut q);
    let ele2 = dequeue(&mut q);
    let ele3 = dequeue(&mut q);
    println!("{}",ele);
    println!("{}", ele1);
    println!("{}", ele2);

    val = 16;
   enqueue(&mut q, val);
    val = 14;
   enqueue(&mut q, val);
    val = 14;
   enqueue(&mut q, val);
    val = 14;
   enqueue(&mut q, val);
    println!("{:?}", q.arr);
    */
}

