#![feature(test)]

fn main() {}

#[cfg(test)]
mod test {
    extern crate test;
    use std::{fs::File, io::BufReader};

    use flate2::read::MultiGzDecoder;
    use test::Bencher;
    use fastq::Parser;

    const NUM: usize = 2000;
    const BUFSIZE: usize = 4096 * 68;

    #[bench]
    fn speed_fxread(b: &mut Bencher) {
        b.iter(|| {
            let file = File::open("example/mini.fq").unwrap();
            let buffer = BufReader::with_capacity(BUFSIZE, file);
            let reader = fxread::FastqReader::new(buffer);
            let num_records = reader.count();
            assert_eq!(num_records, NUM);
        })
    }

    #[bench]
    fn speed_bio(b: &mut Bencher) {
        b.iter(|| {
            let file = File::open("example/mini.fq").unwrap();
            let buffer = BufReader::with_capacity(BUFSIZE, file);
            let reader = bio::io::fastq::Reader::new(buffer);
            let num_records = reader.records().count();
            assert_eq!(num_records, NUM);
        })
    }

    #[bench]
    fn speed_fastq(b: &mut Bencher) {
        b.iter(|| {
            let file = File::open("example/mini.fq").unwrap();
            let buffer = BufReader::with_capacity(BUFSIZE, file);
            let reader = Parser::new(buffer);
            let mut refiter = reader.ref_iter();
            let mut num_records = 0;
            loop {
                match refiter.advance() {
                    Ok(_) => match refiter.get() {
                        Some(_) => num_records += 1,
                        None => break
                        },
                    Err(why) => panic!("{}", why)
                    }
                }
            assert_eq!(num_records, NUM);
            })
    }

    #[bench]
    fn speed_fxread_gzip(b: &mut Bencher) {
        b.iter(|| {
            let file = File::open("example/mini.fq.gz").unwrap();
            let gzip = MultiGzDecoder::new(file);
            let buffer = BufReader::new(gzip);
            let reader = fxread::FastqReader::new(buffer);
            let num_records = reader.count();
            assert_eq!(num_records, NUM);
        })
    }


    #[bench]
    fn speed_bio_gzip(b: &mut Bencher) {
        b.iter(|| {
            let file = File::open("example/mini.fq.gz").unwrap();
            let gzip = MultiGzDecoder::new(file);
            let buffer = BufReader::with_capacity(BUFSIZE, gzip);
            let reader = bio::io::fastq::Reader::new(buffer);
            let num_records = reader.records().count();
            assert_eq!(num_records, NUM);
        })
    }

    #[bench]
    fn speed_fastq_gzip(b: &mut Bencher) {
        b.iter(|| {
            let file = File::open("example/mini.fq.gz").unwrap();
            let gzip = MultiGzDecoder::new(file);
            let buffer = BufReader::with_capacity(BUFSIZE, gzip);
            let reader = Parser::new(buffer);
            let mut refiter = reader.ref_iter();
            let mut num_records = 0;
            loop {
                match refiter.advance() {
                    Ok(_) => match refiter.get() {
                        Some(_) => num_records += 1,
                        None => break
                        },
                    Err(why) => panic!("{}", why)
                    }
                }
            assert_eq!(num_records, NUM);
            })
    }
}
