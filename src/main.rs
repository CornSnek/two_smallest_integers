mod number_factors{
    pub struct NumberFactors{
        number: u64,
        factors: Vec<u64>
    }
    fn print_factors_helper(u64sl:&[u64])->String{
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
    impl NumberFactors{
        pub fn new(number:u64)->Self{
            assert_ne!(number,0,"0 is invalid here");
            Self{
                number,
                factors:{
                    use std::thread;
                    use std::sync::{Arc,Mutex};
                    let number_div_sr=Arc::new(Mutex::new(0u64));
                    let number_div_left_sr=Arc::new(Mutex::new(number));
                    let factors_sr=Arc::new(Mutex::new(vec![1u64]));//Always divisible by 1. Divisors from smallest to largest.
                    let keep_read_sr=Arc::new(Mutex::new(true));
                    let number_div_sr_write=number_div_sr.clone();
                    let number_div_left_sr_write=number_div_left_sr.clone();
                    let factors_sr_write=factors_sr.clone();
                    let keep_read_sr_write=keep_read_sr.clone();
                    let write_handle=thread::spawn(move||{
                        let mut number_div_left=number;
                        let mut number_div=2u64;//Start divisor by 2.
                        while number_div<=number_div_left{//If divisor number_div can still divide into number_cpy.
                            while number_div_left%number_div==0{
                                if let Ok(mut factors)=factors_sr_write.lock(){
                                    factors.push(number_div);
                                }
                                number_div_left/=number_div;
                                if let Ok(mut number_div_left_sr)=number_div_left_sr_write.lock(){
                                    *number_div_left_sr=number_div_left;
                                }
                            }
                            number_div+=1;
                            if let Ok(mut number_div_sr)=number_div_sr_write.lock(){
                                *number_div_sr=number_div;
                            }
                        }
                        if let Ok(mut keep_read)=keep_read_sr_write.lock(){
                            *keep_read=false;
                        }
                    });
                    let number_div_sr_read=number_div_sr.clone();
                    let number_div_left_sr_read=number_div_left_sr.clone();
                    let factors_sr_read=factors_sr.clone();
                    let keep_read_sr_read=keep_read_sr.clone();
                    let read_handle=thread::spawn(move||{
                        let mut keep_read=keep_read_sr_read.lock().unwrap();
                        println!("Finding factors for this number. This may take very long if this number has very large prime number factors or if the number of factors is very large.");
                        while *keep_read{
                            drop(keep_read);
                            if let (Ok(number_div),Ok(number_div_left),Ok(factors))
                                =(number_div_sr_read.lock(),number_div_left_sr_read.lock(),factors_sr_read.lock()){
                                println!("Attempting to divide number {} by {}. Number left: {}",number,number_div,number_div_left);
                                println!("Current factors right now: {}",print_factors_helper(&factors));
                            }
                            thread::sleep(std::time::Duration::from_millis(200));
                            print!("\x1b[2F\x1b[0J");
                            keep_read=keep_read_sr_read.lock().unwrap();
                        }
                        drop(keep_read);
                    });
                    write_handle.join().unwrap();
                    read_handle.join().unwrap();
                    let mut factors_get=Vec::<u64>::new();
                    if let Ok(factors)=factors_sr.lock(){
                        factors_get=factors.clone();
                    };
                    factors_get
                }
            }
        }
        pub fn print_factors(&self){
            println!("Factors for number {} (Including 1): {}",self.number,print_factors_helper(&self.factors));
        }
        pub fn calculate_smallest(&self){
            use std::thread;
            use std::sync::{Arc,Mutex};
            println!("Finding 2 integers a and b, where a*b = {} and |a-b| is at a minimum.",self.number);
            //abs_v,int_lhs,and int_rhs,v_lhs,v_rhs,and a bool to stop reading.
            let srs:Arc<Mutex<(u64,u64,u64,Vec<u64>,Vec<u64>,Vec<bool>,bool)>>=Arc::new(Mutex::new((u64::MAX,1u64,1u64,Vec::new(),Vec::new(),Vec::new(),true)));
            let srs_write=srs.clone();
            let self_cpy=Self{number:self.number,factors:self.factors.clone()};
            let write_handle=thread::spawn(move||{
                let mut abs_v=u64::MAX;
                let mut int_lhs:u64=1;
                let mut int_rhs:u64=1;
                let svi=SplitVectorIter::new(self_cpy.factors.len());
                if self_cpy.number!=1{
                    for vec_split in svi{
                        if let Ok(mut srs_w)=srs_write.lock(){
                            srs_w.5=vec_split.clone();
                        }
                        //for b in &vec_split[..]{print!("{}",if *b{1}else{0}); } println!("");
                        let mut v1:Vec<u64>=Vec::new();
                        let mut v2:Vec<u64>=Vec::new();
                        for (b,factor) in vec_split[..].iter().zip(self_cpy.factors.iter()){
                            if *b{v1.push(*factor);}else{v2.push(*factor);}
                        }
                        let v1_res={
                            let mut res=1;
                            for i in v1.iter(){
                                res*=i;
                            }
                            res
                        };
                        let v2_res={
                            let mut res=1;
                            for i in v2.iter(){
                                res*=i;
                            }
                            res
                        };
                        if (v1_res).abs_diff(v2_res)<abs_v{
                            abs_v=(v1_res).abs_diff(v2_res);
                            int_lhs=v1_res;
                            int_rhs=v2_res;
                            if let Ok(mut srs_w)=srs_write.lock(){
                                srs_w.0=abs_v;
                                srs_w.1=int_lhs;
                                srs_w.2=int_rhs;
                                srs_w.3=v1.clone();
                                srs_w.4=v2.clone();
                            }
                        }
                    }
                }else{
                    abs_v=0;
                    if let Ok(mut srs_w)=srs_write.lock(){
                        srs_w.0=abs_v;
                        srs_w.1=int_lhs;
                        srs_w.2=int_rhs;
                        srs_w.3=vec![1];
                        srs_w.4=vec![1];
                    }
                }
                if let Ok(mut srs_w)=srs_write.lock(){
                    srs_w.6=false;
                }
            });
            let srs_read=srs.clone();
            let read_handle=thread::spawn(move||{
                let mut srs_r=srs_read.lock().unwrap();
                while srs_r.6{
                    let (abs_v,int_lhs,int_rhs,v_lhs,v_rhs,vec_split)=(srs_r.0,srs_r.1,srs_r.2,srs_r.3.clone(),srs_r.4.clone(),srs_r.5.clone());
                    drop(srs_r);
                    use std::io::Write;
                    std::io::stdout().write("Partitioning and multiplying combinations of factors (a uses 1, b uses 0): ".as_bytes()).unwrap();
                    for b in vec_split.iter(){
                        std::io::stdout().write(if *b{b"1"}else{b"0"}).unwrap();
                    }
                    println!("\na: {} := {}\nb: {} := {}\n|a-b|: {}\n(Still in progress)",int_lhs,print_factors_helper(&v_lhs),int_rhs,print_factors_helper(&v_rhs),abs_v);
                    thread::sleep(std::time::Duration::from_millis(100));
                    print!("\x1b[5F\x1b[0J");
                    srs_r=srs_read.lock().unwrap();
                }
                drop(srs_r);
            });
            write_handle.join().unwrap();
            read_handle.join().unwrap();
            if let Ok(srs_get)=srs.lock(){
                println!("a: {} := {}\nb: {} := {}\n|a-b|: {}",srs_get.1,print_factors_helper(&srs_get.3),srs_get.2,print_factors_helper(&srs_get.4),srs_get.0);
            };
            println!("Done!\n");
        }
    }
    //Iterator used to find all combinations for a vector set V into two disjoint non-empty unique subsets A and B.
    //A/B has at least one element, (A U B)=V, and (A Intersect B)=Empty.
    //The unique sets of A and B can be interchanged and is counted as 1 only.
    //Example: If V={1,2,3}, then A,B={2},{1,3} and A,B={1,3},{2} is counted as 1 only.
    //This is valid if size is >=2.
    //There is a special case for size 1, where B can be empty.
    struct SplitVectorIter{
        size: usize,
        trues: usize,
        bool_table: Vec<bool>, //true/false to split into 2 subsets.
        true_i_table: Vec<usize>, //Index of 'true's for bool_table, size is SplitVectorIter::trues: usize.
        first_init: bool, //The first set 1 0 ... 0 0 will not be skipped.
        check_half_last: bool //For an even set, stop iterating where boolean values are flipped.
    }
    impl SplitVectorIter{
        fn new(size: usize)->Self{
            assert_ne!(size,0,"Size must be greater than 0.");
            SplitVectorIter{size,trues:1,
                bool_table:{
                    let mut bt=vec![true]; //Default 0 is true.
                    bt.resize(size,false);
                    bt
                },
                true_i_table:vec![0],
                first_init:false,
                check_half_last:if size!=2{false}else{true} //2 is 1 0 and 0 1. Only iterate 1 0.
            }
        }
    }
    impl Iterator for SplitVectorIter{
        type Item=Vec<bool>;
        fn next(& mut self)->Option<Self::Item>{
            let check_half_last_bt:Option<Vec<bool>>=match self.check_half_last{
                false=>{
                    None
                },
                true=>{
                    Some(self.bool_table.clone())
                }
            };
            if self.first_init{
                let current_i=self.true_i_table[0];
                if current_i!=self.size-1{
                    self.bool_table[current_i]=false;
                    self.bool_table[current_i+1]=true;
                    self.true_i_table[0]+=1;
                }else{
                    let mut last_contig_i:isize=-1;//Always 0 or greater due to self.true_i_table[0]==self.size-1
                    if{//If all boolean values are at the end.
                        let mut is_contig=true;
                        for contig_b in self.bool_table[self.bool_table.len()-self.trues..].iter().rev(){
                            if !*contig_b{
                                is_contig=false;
                                break;
                            }
                            last_contig_i+=1;
                        }
                        is_contig
                    }{//+1 true, -1 false, and move all bools to the left.
                        if self.trues==1&&self.size==1{return None;} //If SplitVectorIter::size is 1.
                        self.trues+=1;
                        if self.trues%(self.size/2)==0{
                            self.check_half_last=true; //Half will be flipped versions of the same combination.
                        }if self.trues==self.size/2+1{
                            return None;
                        }
                        self.true_i_table.push(0);
                        self.bool_table.fill(false);
                        let mut true_i=self.trues;
                        for i in &mut self.true_i_table[..]{
                            true_i-=1;
                            *i=true_i;
                            self.bool_table[*i]=true;
                        }
                    }else{//Move the next_bit not contiguous by 1 and move all the contiguous bits next to next_bit.
                        let next_bit_i=(last_contig_i+1) as usize;
                        let true_bit=&mut self.true_i_table[next_bit_i];
                        self.bool_table[*true_bit]=false;
                        *true_bit+=1;
                        self.bool_table[*true_bit]=true;
                        for (i,contig_i) in (0..=last_contig_i).rev().enumerate(){
                            self.bool_table[self.true_i_table[contig_i as usize]]=false;
                            self.bool_table[self.true_i_table[next_bit_i]+1+i]=true;
                            self.true_i_table[contig_i as usize]=self.true_i_table[next_bit_i]+1+i;
                        }
                    }
                }
            }else{//To get the first value from Self::new
                self.first_init=true;
            }
            /*
            print!("[");
            for (i,true_i) in self.true_i_table[..].iter().enumerate(){
                print!("{}{}",true_i,if i!=self.true_i_table.len()-1{","}else{""});
            }
            println!("]");
            */
            if let Some(bt)=check_half_last_bt{
                let mut all_opposite=true;
                for (b1,b2) in bt.iter().zip(self.bool_table.iter()){
                    if b1==b2{
                        all_opposite=false;
                        break;
                    }
                }
                return if all_opposite{
                    None //The next bool tables are just the opposite and flipped versions.
                }else{
                    Some(self.bool_table.clone())
                };
            }else{
                Some(self.bool_table.clone())
            }
        }
    }
}
fn main()-> std::io::Result<()> {
    loop{
        use std::io::Write;
        let mut input=String::new();
        print!("Type a u64 integer n>0 to find 2 integers a and b such that a*b=n and |a-b| is at a minimum. Type 0 to exit.\n> ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        if let Ok(num)=input.trim().parse::<u64>(){
            if num==0{break;}
            let nf=number_factors::NumberFactors::new(num);
            nf.print_factors();
            nf.calculate_smallest();
        }else{
            input.pop();//Remove \n
            println!("'{}' is not a valid positive u64 integer.\n",input);
            continue;
        }
    }
    Ok(())
}
