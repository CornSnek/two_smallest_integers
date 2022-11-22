mod utils;
pub mod bit_iter;
#[derive(Copy, Clone)]
pub enum CalcOperations{
    StartedFactors,
    StartedIntegerMult,
    Finished,
    Cancelled
}
pub struct NumberFactors{
    number: u64,
    factors: Vec<u64>,
    pub status: CalcOperations
}
impl NumberFactors{
    pub fn new(number:u64)->Self{
        assert_ne!(number,0,"0 is invalid here");
        use std::thread;
        use std::sync::{Arc,Mutex};
        let number_div_sr=Arc::new(Mutex::new(0u64));
        let number_div_left_sr=Arc::new(Mutex::new(number));
        let factors_sr=Arc::new(Mutex::new(vec![1u64]));//Always divisible by 1. Divisors from smallest to largest.
        let keep_read_sr=Arc::new(Mutex::new(true));
        let status_sr=Arc::new(Mutex::new(CalcOperations::StartedFactors));
        let write_handle=thread::spawn({
            let number_div_sr=number_div_sr.clone();
            let number_div_left_sr=number_div_left_sr.clone();
            let factors_sr=factors_sr.clone();
            let keep_read_sr=keep_read_sr.clone();
            let status_sr=status_sr.clone();
            move||{
            let mut number_div_left=number;
            let mut number_div=2u64;//Start divisor by 2.
            while number_div_left%number_div==0{
                if let Ok(mut factors)=factors_sr.lock(){
                    factors.push(number_div);
                }
                number_div_left/=number_div;
                if let Ok(mut number_div_left_sr)=number_div_left_sr.lock(){
                    *number_div_left_sr=number_div_left;
                }
            }
            number_div=3u64;
            if let Ok(mut number_div_sr)=number_div_sr.lock(){
                *number_div_sr=number_div;
            }
            let mut status=status_sr.lock().unwrap();
            while matches!(*status,CalcOperations::StartedFactors)&&number_div<=number_div_left&&number_div_left!=1{//If divisor number_div can still divide into number_cpy.
                drop(status);
                while number_div_left%number_div==0{
                    if let Ok(mut factors)=factors_sr.lock(){
                        factors.push(number_div);
                    }
                    number_div_left/=number_div;
                    if let Ok(mut number_div_left_sr)=number_div_left_sr.lock(){
                        *number_div_left_sr=number_div_left;
                    }
                }
                number_div=utils::next_possible_prime(number_div);
                if number_div*3>=number_div_left&&number_div_left!=1{//If possible prime factors are exhausted down to 3, number_div is just number_div_left then.
                    number_div=number_div_left;
                    number_div_left=1;
                    if let (Ok(mut factors),Ok(mut number_div_left_sr))=(factors_sr.lock(),number_div_left_sr.lock()){
                        factors.push(number_div);
                        *number_div_left_sr=number_div_left;
                    }
                }
                if let Ok(mut number_div_sr)=number_div_sr.lock(){
                    *number_div_sr=number_div;
                }
                status=status_sr.lock().unwrap();
            }
            drop(status);
            if let Ok(mut keep_read)=keep_read_sr.lock(){
                *keep_read=false;
            }}}
        );
        let number_div_sr_read=number_div_sr;
        let number_div_left_sr_read=number_div_left_sr;
        let read_handle=thread::spawn({ 
            let factors_sr=factors_sr.clone();
            let keep_read_sr=keep_read_sr.clone();
            let status_sr=status_sr.clone();
            move||{
            let mut keep_read=keep_read_sr.lock().unwrap();
            let mut status=status_sr.lock().unwrap();
            println!("Finding factors for this number. This may take very long if this number has very large prime number factors or if the number of factors is very large.\n\n\n");
            while matches!(*status,CalcOperations::StartedFactors)&&*keep_read{
                print!("\x1b[3F\x1b[0J");
                drop(keep_read);
                drop(status);
                if let (Ok(number_div),Ok(number_div_left),Ok(factors))
                    =(number_div_sr_read.lock(),number_div_left_sr_read.lock(),factors_sr.lock()){
                    println!("Attempting to divide number {} by {}. Number left: {}",number,number_div,number_div_left);
                    println!("Current factors right now: {}",utils::print_factors_helper(&factors));
                    println!("(Still calculating)");
                }
                thread::sleep(std::time::Duration::from_millis(200));
                keep_read=keep_read_sr.lock().unwrap();
                status=status_sr.lock().unwrap();
            }
            if matches!(*status,CalcOperations::StartedFactors){
                *status=CalcOperations::StartedIntegerMult;//Tell main thread to not cancel when pressing enter.
            }
            if let (Ok(number_div),Ok(number_div_left),Ok(factors))
                =(number_div_sr_read.lock(),number_div_left_sr_read.lock(),factors_sr.lock()){
                print!("\x1b[3F\x1b[0J");
                println!("Attempting to divide number {} by {}. Number left: {}",number,number_div,number_div_left);
                println!("Current factors: {}",utils::print_factors_helper(&factors));
            }
            println!("Press enter to calculate a, b, and |a-b|...");
        }});
        if let Ok(())=utils::do_enter_wait(){
            if let Ok(mut status)=status_sr.lock(){
                if matches!(*status,CalcOperations::StartedFactors){
                    *status=CalcOperations::Cancelled;
                }
            }
        }
        write_handle.join().unwrap();
        read_handle.join().unwrap();
        let factors=factors_sr.lock().unwrap().to_vec();
        let cancelled=*status_sr.lock().unwrap();
        Self{
            number,
            factors,
            status: cancelled
        }
    }
    pub fn calculate_smallest(&self){
        use std::thread;
        use std::sync::{Arc,Mutex};
        println!("Finding 2 integers a and b, where a*b = {} and |a-b| is at a minimum.",self.number);
        //abs_v,int_lhs,and int_rhs,v_lhs,v_rhs,and a bool to stop reading.
        let srs:Arc<Mutex<(u64,u64,u64,Vec<u64>,Vec<u64>,u64,bool)>>=Arc::new(Mutex::new((u64::MAX,1u64,1u64,Vec::new(),Vec::new(),0u64,true)));
        let srs_write=srs.clone();
        let status_sr=Arc::new(Mutex::new(CalcOperations::StartedIntegerMult));
        let self_cpy_write=Self{number:self.number,factors:self.factors.clone(),status:CalcOperations::StartedIntegerMult};
        let status_sr_write=status_sr.clone();
        let write_handle=thread::spawn(move||{
            let mut abs_v=u64::MAX;
            let mut int_lhs:u64=1;
            let mut int_rhs:u64=1;
            let bc_iter=bit_iter::BitCombinationsIter::new(self_cpy_write.factors.len());
            if self_cpy_write.number!=1{
                for num_vec in bc_iter{
                    if let Ok(status)=status_sr_write.lock(){
                        if matches!(*status,CalcOperations::Cancelled){
                            return;
                        }
                    }
                    if let Ok(mut srs_w)=srs_write.lock(){
                        srs_w.5=num_vec;
                    }
                    //for b in &vec_split[..]{print!("{}",if *b{1}else{0}); } println!("");
                    let mut v1:Vec<u64>=Vec::new();
                    let mut v2:Vec<u64>=Vec::new();
                    for (i,factor) in (0..self_cpy_write.factors.len()).zip(self_cpy_write.factors.iter()){
                        if num_vec&(1u64<<i)!=0{v1.push(*factor);}else{v2.push(*factor);}
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
        let vec_size=self.factors.len();
        let status_sr_read=status_sr.clone();
        let read_handle=thread::spawn(move||{
            let mut srs_r=srs_read.lock().unwrap();
            let mut status=status_sr_read.lock().unwrap();
            while matches!(*status,CalcOperations::StartedIntegerMult)&&srs_r.6{
                drop(status);
                let (abs_v,int_lhs,int_rhs,v_lhs,v_rhs,vec_split)=(srs_r.0,srs_r.1,srs_r.2,srs_r.3.clone(),srs_r.4.clone(),srs_r.5);
                drop(srs_r);
                use std::io::Write;
                std::io::stdout().write_all("Partitioning and multiplying combinations of factors (a uses 1, b uses 0): ".as_bytes()).unwrap();
                for i in 0..vec_size{
                    std::io::stdout().write_all(if vec_split&(1u64<<i)!=0{b"1"}else{b"0"}).unwrap();
                }
                println!("\na: {} := {}\nb: {} := {}\n|a-b|: {}\n(Still in progress)",int_lhs,utils::print_factors_helper(&v_lhs),int_rhs,utils::print_factors_helper(&v_rhs),abs_v);
                thread::sleep(std::time::Duration::from_millis(100));
                srs_r=srs_read.lock().unwrap();
                status=status_sr_read.lock().unwrap();
                if srs_r.6&&matches!(*status,CalcOperations::StartedIntegerMult){ print!("\x1b[5F\x1b[0J"); };
            }
            if matches!(*status,CalcOperations::StartedIntegerMult){
                *status=CalcOperations::Finished;
                println!("Finished! Press enter to see results.");
            }
            drop(status);
            drop(srs_r);
        });
        if let Ok(())=utils::do_enter_wait(){
            if let Ok(mut status)=status_sr.lock(){
                if matches!(*status,CalcOperations::StartedIntegerMult){
                    *status=CalcOperations::Cancelled;
                    println!("Cancelled by pressing enter.");
                }
            }
        }
        write_handle.join().unwrap();
        read_handle.join().unwrap();
        let status=status_sr.lock().unwrap();
        if matches!(*status,CalcOperations::Cancelled){ return; }
        if let Ok(srs_get)=srs.lock(){
            println!("a: {} := {}\nb: {} := {}\n|a-b|: {}",srs_get.1,utils::print_factors_helper(&srs_get.3),srs_get.2,utils::print_factors_helper(&srs_get.4),srs_get.0);
        };
        println!();
    }
}