mod number_factors;
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
            if let number_factors::CalcOperations::Done=nf.status{
                nf.calculate_smallest();
            }else{
                println!("Cancelled when pressing enter.");
            }
        }else{
            input.pop();//Remove \n
            println!("'{}' is not a valid positive u64 integer.\n",input);
            continue;
        }
    }
    Ok(())
}
