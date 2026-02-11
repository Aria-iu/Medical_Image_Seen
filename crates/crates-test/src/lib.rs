#![allow(unused)]

use std::any::Any;

use nifti::*;
use nifti::Result;


pub fn test1()
{
    let hdr1 = NiftiHeader::from_file("C:/Users/zyc/Desktop/Hospital/ct_nifti_results/17106690_2_CT.nii.gz").unwrap();

    print!("nii file dim is :{}\n", hdr1.dimensionality().unwrap());
    print!("nii file dims are : {:?}\n",hdr1.dim().unwrap());
    print!("nii file xyzt_to_space is : {:?}\n",hdr1.xyzt_to_space().unwrap());
    print!("nii file xyzt_to_time is : {:?}\n",hdr1.xyzt_to_time().unwrap());
    print!("nii file affine is : {:?}\n",hdr1.affine::<f64>());
    print!("nii file slice_order is : {:?}\n",hdr1.slice_order());
    print!("nii file intent is : {:?}\n",hdr1.intent());
    print!("nii file qform is : {:?}\n",hdr1.qform());
    print!("nii file sform is : {:?}\n",hdr1.sform());
}

pub fn test2()
{
    const FILE_NAME: &str = "C:/Users/zyc/Desktop/Hospital/ct_nifti_results/17106690_2_CT.nii.gz";
    let obj = ReaderStreamedOptions::new().read_file(FILE_NAME).unwrap();
    let volume = obj.volume();
    println!("nii file volum data_type is :{:?}",volume.data_type());
    println!("nii file volum dim is :{:?}",volume.dim());
}