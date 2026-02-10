#![allow(unused)]

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
}