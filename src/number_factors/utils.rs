pub fn do_enter_wait()-> std::io::Result<()> {
    let mut temp=String::new();
    std::io::stdin().read_line(&mut temp)?;
    drop(temp);//Don't want the stdin from line.
    Ok(())
}
pub fn next_possible_prime(num:u64)->u64{
    if (num+2)%3!=0{ num+2 }else{ num+4 }
}
pub fn print_factors_helper(u64sl:&[u64])->String{
    let mut str:String=String::new();
    let mut vec_freq:Vec<(u64,usize)>=Vec::new();
    for v in u64sl{
        if let Some((v_cmp,count))=vec_freq.last_mut(){
            if *v==*v_cmp{//Increase frequency of existent factor by 1
                *count+=1;
            }else{
                vec_freq.push((*v,1));
            }
        }else{
            vec_freq.push((*v,1));
        }
    }
    for (i,(v,count)) in vec_freq.iter().enumerate(){
        let exp_str=if *count>1{
            let mut str=count.to_string();
            str.insert_str(0,"^");
            str
        }else{
            String::from("")
        };
        str.push_str(format!("{}{}{}",v,exp_str,if i<vec_freq.len()-1{" * "}else{""}).as_str());
    }
    str
}