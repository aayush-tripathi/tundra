#[doc(hidden)] // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:161
macro_rules! isle_assembler_methods {
    () => {
        fn x64_addb_i_raw(&mut self, al: Gpr, imm8: u8) -> AssemblerOutputs {
            let al = self.convert_gpr_to_assembler_fixed_read_write_gpr(al); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addb_i::new(al.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = al.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_addw_i_raw(&mut self, ax: Gpr, imm16: u16) -> AssemblerOutputs {
            let ax = self.convert_gpr_to_assembler_fixed_read_write_gpr(ax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addw_i::new(ax.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = ax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_addl_i_raw(&mut self, eax: Gpr, imm32: u32) -> AssemblerOutputs {
            let eax = self.convert_gpr_to_assembler_fixed_read_write_gpr(eax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addl_i::new(eax.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = eax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_addq_i_sxl_raw(&mut self, rax: Gpr, imm32: i32) -> AssemblerOutputs {
            let rax = self.convert_gpr_to_assembler_fixed_read_write_gpr(rax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addq_i_sxl::new(rax.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = rax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_addb_mi_raw(&mut self, rm8: &GprMem, imm8: u8) -> AssemblerOutputs {
            let rm8 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addb_mi::new(rm8.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm8 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_addw_mi_raw(&mut self, rm16: &GprMem, imm16: u16) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addw_mi::new(rm16.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_addl_mi_raw(&mut self, rm32: &GprMem, imm32: u32) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addl_mi::new(rm32.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_addq_mi_sxl_raw(&mut self, rm64: &GprMem, imm32: i32) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addq_mi_sxl::new(rm64.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_addl_mi_sxb_raw(&mut self, rm32: &GprMem, imm8: i8) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addl_mi_sxb::new(rm32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_addq_mi_sxb_raw(&mut self, rm64: &GprMem, imm8: i8) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addq_mi_sxb::new(rm64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_addb_mr_raw(&mut self, rm8: &GprMem, r8: Gpr) -> AssemblerOutputs {
            let rm8 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r8 = cranelift_assembler_x64::Gpr::new(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addb_mr::new(rm8.clone(), r8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm8 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_addw_mr_raw(&mut self, rm16: &GprMem, r16: Gpr) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addw_mr::new(rm16.clone(), r16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_addl_mr_raw(&mut self, rm32: &GprMem, r32: Gpr) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addl_mr::new(rm32.clone(), r32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_addq_mr_raw(&mut self, rm64: &GprMem, r64: Gpr) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addq_mr::new(rm64.clone(), r64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_addb_rm_raw(&mut self, r8: Gpr, rm8: &GprMem) -> AssemblerOutputs {
            let r8 = self.convert_gpr_to_assembler_read_write_gpr(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm8 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addb_rm::new(r8.clone(), rm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r8.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_addw_rm_raw(&mut self, r16: Gpr, rm16: &GprMem) -> AssemblerOutputs {
            let r16 = self.convert_gpr_to_assembler_read_write_gpr(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm16 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addw_rm::new(r16.clone(), rm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r16.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_addl_rm_raw(&mut self, r32: Gpr, rm32: &GprMem) -> AssemblerOutputs {
            let r32 = self.convert_gpr_to_assembler_read_write_gpr(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm32 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addl_rm::new(r32.clone(), rm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r32.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_addq_rm_raw(&mut self, r64: Gpr, rm64: &GprMem) -> AssemblerOutputs {
            let r64 = self.convert_gpr_to_assembler_read_write_gpr(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm64 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addq_rm::new(r64.clone(), rm64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r64.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_adcb_i_raw(&mut self, al: Gpr, imm8: u8) -> AssemblerOutputs {
            let al = self.convert_gpr_to_assembler_fixed_read_write_gpr(al); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcb_i::new(al.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = al.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_adcw_i_raw(&mut self, ax: Gpr, imm16: u16) -> AssemblerOutputs {
            let ax = self.convert_gpr_to_assembler_fixed_read_write_gpr(ax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcw_i::new(ax.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = ax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_adcl_i_raw(&mut self, eax: Gpr, imm32: u32) -> AssemblerOutputs {
            let eax = self.convert_gpr_to_assembler_fixed_read_write_gpr(eax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcl_i::new(eax.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = eax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_adcq_i_sxl_raw(&mut self, rax: Gpr, imm32: i32) -> AssemblerOutputs {
            let rax = self.convert_gpr_to_assembler_fixed_read_write_gpr(rax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcq_i_sxl::new(rax.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = rax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_adcb_mi_raw(&mut self, rm8: &GprMem, imm8: u8) -> AssemblerOutputs {
            let rm8 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcb_mi::new(rm8.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm8 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_adcw_mi_raw(&mut self, rm16: &GprMem, imm16: u16) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcw_mi::new(rm16.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_adcl_mi_raw(&mut self, rm32: &GprMem, imm32: u32) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcl_mi::new(rm32.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_adcq_mi_sxl_raw(&mut self, rm64: &GprMem, imm32: i32) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcq_mi_sxl::new(rm64.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_adcl_mi_sxb_raw(&mut self, rm32: &GprMem, imm8: i8) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcl_mi_sxb::new(rm32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_adcq_mi_sxb_raw(&mut self, rm64: &GprMem, imm8: i8) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcq_mi_sxb::new(rm64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_adcb_mr_raw(&mut self, rm8: &GprMem, r8: Gpr) -> AssemblerOutputs {
            let rm8 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r8 = cranelift_assembler_x64::Gpr::new(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcb_mr::new(rm8.clone(), r8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm8 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_adcw_mr_raw(&mut self, rm16: &GprMem, r16: Gpr) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcw_mr::new(rm16.clone(), r16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_adcl_mr_raw(&mut self, rm32: &GprMem, r32: Gpr) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcl_mr::new(rm32.clone(), r32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_adcq_mr_raw(&mut self, rm64: &GprMem, r64: Gpr) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcq_mr::new(rm64.clone(), r64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_adcb_rm_raw(&mut self, r8: Gpr, rm8: &GprMem) -> AssemblerOutputs {
            let r8 = self.convert_gpr_to_assembler_read_write_gpr(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm8 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcb_rm::new(r8.clone(), rm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r8.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_adcw_rm_raw(&mut self, r16: Gpr, rm16: &GprMem) -> AssemblerOutputs {
            let r16 = self.convert_gpr_to_assembler_read_write_gpr(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm16 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcw_rm::new(r16.clone(), rm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r16.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_adcl_rm_raw(&mut self, r32: Gpr, rm32: &GprMem) -> AssemblerOutputs {
            let r32 = self.convert_gpr_to_assembler_read_write_gpr(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm32 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcl_rm::new(r32.clone(), rm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r32.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_adcq_rm_raw(&mut self, r64: Gpr, rm64: &GprMem) -> AssemblerOutputs {
            let r64 = self.convert_gpr_to_assembler_read_write_gpr(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm64 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::adcq_rm::new(r64.clone(), rm64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r64.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_lock_addb_mi_raw(&mut self, m8: &Amode, imm8: u8) -> AssemblerOutputs {
            let m8 = self.convert_amode_to_assembler_amode(m8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_addb_mi::new(m8.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_addw_mi_raw(&mut self, m16: &Amode, imm16: u16) -> AssemblerOutputs {
            let m16 = self.convert_amode_to_assembler_amode(m16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_addw_mi::new(m16.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_addl_mi_raw(&mut self, m32: &Amode, imm32: u32) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_addl_mi::new(m32.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_addq_mi_sxl_raw(&mut self, m64: &Amode, imm32: i32) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_addq_mi_sxl::new(m64.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_addl_mi_sxb_raw(&mut self, m32: &Amode, imm8: i8) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_addl_mi_sxb::new(m32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_addq_mi_sxb_raw(&mut self, m64: &Amode, imm8: i8) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_addq_mi_sxb::new(m64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_addb_mr_raw(&mut self, m8: &Amode, r8: Gpr) -> AssemblerOutputs {
            let m8 = self.convert_amode_to_assembler_amode(m8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r8 = cranelift_assembler_x64::Gpr::new(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_addb_mr::new(m8.clone(), r8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_addw_mr_raw(&mut self, m16: &Amode, r16: Gpr) -> AssemblerOutputs {
            let m16 = self.convert_amode_to_assembler_amode(m16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_addw_mr::new(m16.clone(), r16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_addl_mr_raw(&mut self, m32: &Amode, r32: Gpr) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_addl_mr::new(m32.clone(), r32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_addq_mr_raw(&mut self, m64: &Amode, r64: Gpr) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_addq_mr::new(m64.clone(), r64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_adcb_mi_raw(&mut self, m8: &Amode, imm8: u8) -> AssemblerOutputs {
            let m8 = self.convert_amode_to_assembler_amode(m8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_adcb_mi::new(m8.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_adcw_mi_raw(&mut self, m16: &Amode, imm16: u16) -> AssemblerOutputs {
            let m16 = self.convert_amode_to_assembler_amode(m16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_adcw_mi::new(m16.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_adcl_mi_raw(&mut self, m32: &Amode, imm32: u32) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_adcl_mi::new(m32.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_adcq_mi_sxl_raw(&mut self, m64: &Amode, imm32: i32) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_adcq_mi_sxl::new(m64.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_adcl_mi_sxb_raw(&mut self, m32: &Amode, imm8: i8) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_adcl_mi_sxb::new(m32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_adcq_mi_sxb_raw(&mut self, m64: &Amode, imm8: i8) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_adcq_mi_sxb::new(m64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_adcb_mr_raw(&mut self, m8: &Amode, r8: Gpr) -> AssemblerOutputs {
            let m8 = self.convert_amode_to_assembler_amode(m8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r8 = cranelift_assembler_x64::Gpr::new(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_adcb_mr::new(m8.clone(), r8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_adcw_mr_raw(&mut self, m16: &Amode, r16: Gpr) -> AssemblerOutputs {
            let m16 = self.convert_amode_to_assembler_amode(m16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_adcw_mr::new(m16.clone(), r16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_adcl_mr_raw(&mut self, m32: &Amode, r32: Gpr) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_adcl_mr::new(m32.clone(), r32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_adcq_mr_raw(&mut self, m64: &Amode, r64: Gpr) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_adcq_mr::new(m64.clone(), r64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_addss_a_raw(&mut self, xmm: Xmm, xmm_m32: &XmmMem) -> AssemblerOutputs {
            let xmm = self.convert_xmm_to_assembler_read_write_xmm(xmm); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let xmm_m32 = self.convert_xmm_mem_to_assembler_read_xmm_mem(xmm_m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addss_a::new(xmm.clone(), xmm_m32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let xmm = xmm.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetXmm { inst, xmm }
        }
        fn x64_addsd_a_raw(&mut self, xmm: Xmm, xmm_m64: &XmmMem) -> AssemblerOutputs {
            let xmm = self.convert_xmm_to_assembler_read_write_xmm(xmm); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let xmm_m64 = self.convert_xmm_mem_to_assembler_read_xmm_mem(xmm_m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addsd_a::new(xmm.clone(), xmm_m64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let xmm = xmm.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetXmm { inst, xmm }
        }
        fn x64_addps_a_raw(&mut self, xmm: Xmm, xmm_m128: &XmmMemAligned) -> AssemblerOutputs {
            let xmm = self.convert_xmm_to_assembler_read_write_xmm(xmm); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let xmm_m128 = self.convert_xmm_mem_to_assembler_read_xmm_mem_aligned(xmm_m128); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addps_a::new(xmm.clone(), xmm_m128.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let xmm = xmm.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetXmm { inst, xmm }
        }
        fn x64_addpd_a_raw(&mut self, xmm: Xmm, xmm_m128: &XmmMemAligned) -> AssemblerOutputs {
            let xmm = self.convert_xmm_to_assembler_read_write_xmm(xmm); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let xmm_m128 = self.convert_xmm_mem_to_assembler_read_xmm_mem_aligned(xmm_m128); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::addpd_a::new(xmm.clone(), xmm_m128.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let xmm = xmm.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetXmm { inst, xmm }
        }
        fn x64_andb_i_raw(&mut self, al: Gpr, imm8: u8) -> AssemblerOutputs {
            let al = self.convert_gpr_to_assembler_fixed_read_write_gpr(al); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andb_i::new(al.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = al.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_andw_i_raw(&mut self, ax: Gpr, imm16: u16) -> AssemblerOutputs {
            let ax = self.convert_gpr_to_assembler_fixed_read_write_gpr(ax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andw_i::new(ax.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = ax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_andl_i_raw(&mut self, eax: Gpr, imm32: u32) -> AssemblerOutputs {
            let eax = self.convert_gpr_to_assembler_fixed_read_write_gpr(eax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andl_i::new(eax.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = eax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_andq_i_sxl_raw(&mut self, rax: Gpr, imm32: i32) -> AssemblerOutputs {
            let rax = self.convert_gpr_to_assembler_fixed_read_write_gpr(rax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andq_i_sxl::new(rax.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = rax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_andb_mi_raw(&mut self, rm8: &GprMem, imm8: u8) -> AssemblerOutputs {
            let rm8 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andb_mi::new(rm8.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm8 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_andw_mi_raw(&mut self, rm16: &GprMem, imm16: u16) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andw_mi::new(rm16.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_andl_mi_raw(&mut self, rm32: &GprMem, imm32: u32) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andl_mi::new(rm32.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_andq_mi_sxl_raw(&mut self, rm64: &GprMem, imm32: i32) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andq_mi_sxl::new(rm64.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_andl_mi_sxb_raw(&mut self, rm32: &GprMem, imm8: i8) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andl_mi_sxb::new(rm32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_andq_mi_sxb_raw(&mut self, rm64: &GprMem, imm8: i8) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andq_mi_sxb::new(rm64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_andb_mr_raw(&mut self, rm8: &GprMem, r8: Gpr) -> AssemblerOutputs {
            let rm8 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r8 = cranelift_assembler_x64::Gpr::new(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andb_mr::new(rm8.clone(), r8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm8 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_andw_mr_raw(&mut self, rm16: &GprMem, r16: Gpr) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andw_mr::new(rm16.clone(), r16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_andl_mr_raw(&mut self, rm32: &GprMem, r32: Gpr) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andl_mr::new(rm32.clone(), r32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_andq_mr_raw(&mut self, rm64: &GprMem, r64: Gpr) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andq_mr::new(rm64.clone(), r64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_andb_rm_raw(&mut self, r8: Gpr, rm8: &GprMem) -> AssemblerOutputs {
            let r8 = self.convert_gpr_to_assembler_read_write_gpr(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm8 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andb_rm::new(r8.clone(), rm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r8.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_andw_rm_raw(&mut self, r16: Gpr, rm16: &GprMem) -> AssemblerOutputs {
            let r16 = self.convert_gpr_to_assembler_read_write_gpr(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm16 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andw_rm::new(r16.clone(), rm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r16.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_andl_rm_raw(&mut self, r32: Gpr, rm32: &GprMem) -> AssemblerOutputs {
            let r32 = self.convert_gpr_to_assembler_read_write_gpr(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm32 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andl_rm::new(r32.clone(), rm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r32.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_andq_rm_raw(&mut self, r64: Gpr, rm64: &GprMem) -> AssemblerOutputs {
            let r64 = self.convert_gpr_to_assembler_read_write_gpr(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm64 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andq_rm::new(r64.clone(), rm64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r64.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_lock_andb_mi_raw(&mut self, m8: &Amode, imm8: u8) -> AssemblerOutputs {
            let m8 = self.convert_amode_to_assembler_amode(m8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_andb_mi::new(m8.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_andw_mi_raw(&mut self, m16: &Amode, imm16: u16) -> AssemblerOutputs {
            let m16 = self.convert_amode_to_assembler_amode(m16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_andw_mi::new(m16.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_andl_mi_raw(&mut self, m32: &Amode, imm32: u32) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_andl_mi::new(m32.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_andq_mi_sxl_raw(&mut self, m64: &Amode, imm32: i32) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_andq_mi_sxl::new(m64.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_andl_mi_sxb_raw(&mut self, m32: &Amode, imm8: i8) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_andl_mi_sxb::new(m32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_andq_mi_sxb_raw(&mut self, m64: &Amode, imm8: i8) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_andq_mi_sxb::new(m64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_andb_mr_raw(&mut self, m8: &Amode, r8: Gpr) -> AssemblerOutputs {
            let m8 = self.convert_amode_to_assembler_amode(m8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r8 = cranelift_assembler_x64::Gpr::new(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_andb_mr::new(m8.clone(), r8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_andw_mr_raw(&mut self, m16: &Amode, r16: Gpr) -> AssemblerOutputs {
            let m16 = self.convert_amode_to_assembler_amode(m16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_andw_mr::new(m16.clone(), r16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_andl_mr_raw(&mut self, m32: &Amode, r32: Gpr) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_andl_mr::new(m32.clone(), r32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_andq_mr_raw(&mut self, m64: &Amode, r64: Gpr) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_andq_mr::new(m64.clone(), r64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_andps_a_raw(&mut self, xmm: Xmm, xmm_m128: &XmmMemAligned) -> AssemblerOutputs {
            let xmm = self.convert_xmm_to_assembler_read_write_xmm(xmm); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let xmm_m128 = self.convert_xmm_mem_to_assembler_read_xmm_mem_aligned(xmm_m128); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andps_a::new(xmm.clone(), xmm_m128.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let xmm = xmm.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetXmm { inst, xmm }
        }
        fn x64_andpd_a_raw(&mut self, xmm: Xmm, xmm_m128: &XmmMemAligned) -> AssemblerOutputs {
            let xmm = self.convert_xmm_to_assembler_read_write_xmm(xmm); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let xmm_m128 = self.convert_xmm_mem_to_assembler_read_xmm_mem_aligned(xmm_m128); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::andpd_a::new(xmm.clone(), xmm_m128.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let xmm = xmm.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetXmm { inst, xmm }
        }
        fn x64_orb_i_raw(&mut self, al: Gpr, imm8: u8) -> AssemblerOutputs {
            let al = self.convert_gpr_to_assembler_fixed_read_write_gpr(al); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orb_i::new(al.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = al.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_orw_i_raw(&mut self, ax: Gpr, imm16: u16) -> AssemblerOutputs {
            let ax = self.convert_gpr_to_assembler_fixed_read_write_gpr(ax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orw_i::new(ax.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = ax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_orl_i_raw(&mut self, eax: Gpr, imm32: u32) -> AssemblerOutputs {
            let eax = self.convert_gpr_to_assembler_fixed_read_write_gpr(eax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orl_i::new(eax.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = eax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_orq_i_sxl_raw(&mut self, rax: Gpr, imm32: i32) -> AssemblerOutputs {
            let rax = self.convert_gpr_to_assembler_fixed_read_write_gpr(rax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orq_i_sxl::new(rax.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = rax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_orb_mi_raw(&mut self, rm8: &GprMem, imm8: u8) -> AssemblerOutputs {
            let rm8 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orb_mi::new(rm8.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm8 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_orw_mi_raw(&mut self, rm16: &GprMem, imm16: u16) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orw_mi::new(rm16.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_orl_mi_raw(&mut self, rm32: &GprMem, imm32: u32) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orl_mi::new(rm32.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_orq_mi_sxl_raw(&mut self, rm64: &GprMem, imm32: i32) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orq_mi_sxl::new(rm64.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_orl_mi_sxb_raw(&mut self, rm32: &GprMem, imm8: i8) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orl_mi_sxb::new(rm32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_orq_mi_sxb_raw(&mut self, rm64: &GprMem, imm8: i8) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orq_mi_sxb::new(rm64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_orb_mr_raw(&mut self, rm8: &GprMem, r8: Gpr) -> AssemblerOutputs {
            let rm8 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r8 = cranelift_assembler_x64::Gpr::new(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orb_mr::new(rm8.clone(), r8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm8 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_orw_mr_raw(&mut self, rm16: &GprMem, r16: Gpr) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orw_mr::new(rm16.clone(), r16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_orl_mr_raw(&mut self, rm32: &GprMem, r32: Gpr) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orl_mr::new(rm32.clone(), r32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_orq_mr_raw(&mut self, rm64: &GprMem, r64: Gpr) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orq_mr::new(rm64.clone(), r64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_orb_rm_raw(&mut self, r8: Gpr, rm8: &GprMem) -> AssemblerOutputs {
            let r8 = self.convert_gpr_to_assembler_read_write_gpr(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm8 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orb_rm::new(r8.clone(), rm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r8.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_orw_rm_raw(&mut self, r16: Gpr, rm16: &GprMem) -> AssemblerOutputs {
            let r16 = self.convert_gpr_to_assembler_read_write_gpr(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm16 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orw_rm::new(r16.clone(), rm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r16.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_orl_rm_raw(&mut self, r32: Gpr, rm32: &GprMem) -> AssemblerOutputs {
            let r32 = self.convert_gpr_to_assembler_read_write_gpr(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm32 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orl_rm::new(r32.clone(), rm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r32.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_orq_rm_raw(&mut self, r64: Gpr, rm64: &GprMem) -> AssemblerOutputs {
            let r64 = self.convert_gpr_to_assembler_read_write_gpr(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm64 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orq_rm::new(r64.clone(), rm64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r64.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_lock_orb_mi_raw(&mut self, m8: &Amode, imm8: u8) -> AssemblerOutputs {
            let m8 = self.convert_amode_to_assembler_amode(m8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_orb_mi::new(m8.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_orw_mi_raw(&mut self, m16: &Amode, imm16: u16) -> AssemblerOutputs {
            let m16 = self.convert_amode_to_assembler_amode(m16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_orw_mi::new(m16.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_orl_mi_raw(&mut self, m32: &Amode, imm32: u32) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_orl_mi::new(m32.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_orq_mi_sxl_raw(&mut self, m64: &Amode, imm32: i32) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_orq_mi_sxl::new(m64.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_orl_mi_sxb_raw(&mut self, m32: &Amode, imm8: i8) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_orl_mi_sxb::new(m32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_orq_mi_sxb_raw(&mut self, m64: &Amode, imm8: i8) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_orq_mi_sxb::new(m64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_orb_mr_raw(&mut self, m8: &Amode, r8: Gpr) -> AssemblerOutputs {
            let m8 = self.convert_amode_to_assembler_amode(m8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r8 = cranelift_assembler_x64::Gpr::new(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_orb_mr::new(m8.clone(), r8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_orw_mr_raw(&mut self, m16: &Amode, r16: Gpr) -> AssemblerOutputs {
            let m16 = self.convert_amode_to_assembler_amode(m16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_orw_mr::new(m16.clone(), r16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_orl_mr_raw(&mut self, m32: &Amode, r32: Gpr) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_orl_mr::new(m32.clone(), r32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_orq_mr_raw(&mut self, m64: &Amode, r64: Gpr) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_orq_mr::new(m64.clone(), r64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_orps_a_raw(&mut self, xmm: Xmm, xmm_m128: &XmmMemAligned) -> AssemblerOutputs {
            let xmm = self.convert_xmm_to_assembler_read_write_xmm(xmm); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let xmm_m128 = self.convert_xmm_mem_to_assembler_read_xmm_mem_aligned(xmm_m128); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orps_a::new(xmm.clone(), xmm_m128.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let xmm = xmm.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetXmm { inst, xmm }
        }
        fn x64_orpd_a_raw(&mut self, xmm: Xmm, xmm_m128: &XmmMemAligned) -> AssemblerOutputs {
            let xmm = self.convert_xmm_to_assembler_read_write_xmm(xmm); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let xmm_m128 = self.convert_xmm_mem_to_assembler_read_xmm_mem_aligned(xmm_m128); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::orpd_a::new(xmm.clone(), xmm_m128.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let xmm = xmm.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetXmm { inst, xmm }
        }
        fn x64_shldw_mri_raw(&mut self, rm16: &GprMem, r16: Gpr, imm8: u8) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::shldw_mri::new(rm16.clone(), r16.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_shldw_mrc_raw(&mut self, rm16: &GprMem, r16: Gpr, cl: Gpr) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let cl = cranelift_assembler_x64::Fixed(cl); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::shldw_mrc::new(rm16.clone(), r16.clone(), cl.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_shldl_mri_raw(&mut self, rm32: &GprMem, r32: Gpr, imm8: u8) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::shldl_mri::new(rm32.clone(), r32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_shldq_mri_raw(&mut self, rm64: &GprMem, r64: Gpr, imm8: u8) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::shldq_mri::new(rm64.clone(), r64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_shldl_mrc_raw(&mut self, rm32: &GprMem, r32: Gpr, cl: Gpr) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let cl = cranelift_assembler_x64::Fixed(cl); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::shldl_mrc::new(rm32.clone(), r32.clone(), cl.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_shldq_mrc_raw(&mut self, rm64: &GprMem, r64: Gpr, cl: Gpr) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let cl = cranelift_assembler_x64::Fixed(cl); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::shldq_mrc::new(rm64.clone(), r64.clone(), cl.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_subb_i_raw(&mut self, al: Gpr, imm8: u8) -> AssemblerOutputs {
            let al = self.convert_gpr_to_assembler_fixed_read_write_gpr(al); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subb_i::new(al.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = al.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_subw_i_raw(&mut self, ax: Gpr, imm16: u16) -> AssemblerOutputs {
            let ax = self.convert_gpr_to_assembler_fixed_read_write_gpr(ax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subw_i::new(ax.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = ax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_subl_i_raw(&mut self, eax: Gpr, imm32: u32) -> AssemblerOutputs {
            let eax = self.convert_gpr_to_assembler_fixed_read_write_gpr(eax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subl_i::new(eax.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = eax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_subq_i_sxl_raw(&mut self, rax: Gpr, imm32: i32) -> AssemblerOutputs {
            let rax = self.convert_gpr_to_assembler_fixed_read_write_gpr(rax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subq_i_sxl::new(rax.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = rax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_subb_mi_raw(&mut self, rm8: &GprMem, imm8: u8) -> AssemblerOutputs {
            let rm8 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subb_mi::new(rm8.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm8 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_subw_mi_raw(&mut self, rm16: &GprMem, imm16: u16) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subw_mi::new(rm16.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_subl_mi_raw(&mut self, rm32: &GprMem, imm32: u32) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subl_mi::new(rm32.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_subq_mi_sxl_raw(&mut self, rm64: &GprMem, imm32: i32) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subq_mi_sxl::new(rm64.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_subl_mi_sxb_raw(&mut self, rm32: &GprMem, imm8: i8) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subl_mi_sxb::new(rm32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_subq_mi_sxb_raw(&mut self, rm64: &GprMem, imm8: i8) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subq_mi_sxb::new(rm64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_subb_mr_raw(&mut self, rm8: &GprMem, r8: Gpr) -> AssemblerOutputs {
            let rm8 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r8 = cranelift_assembler_x64::Gpr::new(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subb_mr::new(rm8.clone(), r8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm8 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_subw_mr_raw(&mut self, rm16: &GprMem, r16: Gpr) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subw_mr::new(rm16.clone(), r16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_subl_mr_raw(&mut self, rm32: &GprMem, r32: Gpr) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subl_mr::new(rm32.clone(), r32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_subq_mr_raw(&mut self, rm64: &GprMem, r64: Gpr) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subq_mr::new(rm64.clone(), r64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_subb_rm_raw(&mut self, r8: Gpr, rm8: &GprMem) -> AssemblerOutputs {
            let r8 = self.convert_gpr_to_assembler_read_write_gpr(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm8 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subb_rm::new(r8.clone(), rm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r8.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_subw_rm_raw(&mut self, r16: Gpr, rm16: &GprMem) -> AssemblerOutputs {
            let r16 = self.convert_gpr_to_assembler_read_write_gpr(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm16 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subw_rm::new(r16.clone(), rm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r16.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_subl_rm_raw(&mut self, r32: Gpr, rm32: &GprMem) -> AssemblerOutputs {
            let r32 = self.convert_gpr_to_assembler_read_write_gpr(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm32 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subl_rm::new(r32.clone(), rm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r32.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_subq_rm_raw(&mut self, r64: Gpr, rm64: &GprMem) -> AssemblerOutputs {
            let r64 = self.convert_gpr_to_assembler_read_write_gpr(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm64 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subq_rm::new(r64.clone(), rm64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r64.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_sbbb_i_raw(&mut self, al: Gpr, imm8: u8) -> AssemblerOutputs {
            let al = self.convert_gpr_to_assembler_fixed_read_write_gpr(al); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbb_i::new(al.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = al.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_sbbw_i_raw(&mut self, ax: Gpr, imm16: u16) -> AssemblerOutputs {
            let ax = self.convert_gpr_to_assembler_fixed_read_write_gpr(ax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbw_i::new(ax.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = ax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_sbbl_i_raw(&mut self, eax: Gpr, imm32: u32) -> AssemblerOutputs {
            let eax = self.convert_gpr_to_assembler_fixed_read_write_gpr(eax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbl_i::new(eax.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = eax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_sbbq_i_sxl_raw(&mut self, rax: Gpr, imm32: i32) -> AssemblerOutputs {
            let rax = self.convert_gpr_to_assembler_fixed_read_write_gpr(rax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbq_i_sxl::new(rax.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = rax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_sbbb_mi_raw(&mut self, rm8: &GprMem, imm8: u8) -> AssemblerOutputs {
            let rm8 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbb_mi::new(rm8.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm8 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_sbbw_mi_raw(&mut self, rm16: &GprMem, imm16: u16) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbw_mi::new(rm16.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_sbbl_mi_raw(&mut self, rm32: &GprMem, imm32: u32) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbl_mi::new(rm32.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_sbbq_mi_sxl_raw(&mut self, rm64: &GprMem, imm32: i32) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbq_mi_sxl::new(rm64.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_sbbl_mi_sxb_raw(&mut self, rm32: &GprMem, imm8: i8) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbl_mi_sxb::new(rm32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_sbbq_mi_sxb_raw(&mut self, rm64: &GprMem, imm8: i8) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbq_mi_sxb::new(rm64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_sbbb_mr_raw(&mut self, rm8: &GprMem, r8: Gpr) -> AssemblerOutputs {
            let rm8 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r8 = cranelift_assembler_x64::Gpr::new(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbb_mr::new(rm8.clone(), r8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm8 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_sbbw_mr_raw(&mut self, rm16: &GprMem, r16: Gpr) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbw_mr::new(rm16.clone(), r16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_sbbl_mr_raw(&mut self, rm32: &GprMem, r32: Gpr) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbl_mr::new(rm32.clone(), r32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_sbbq_mr_raw(&mut self, rm64: &GprMem, r64: Gpr) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbq_mr::new(rm64.clone(), r64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_sbbb_rm_raw(&mut self, r8: Gpr, rm8: &GprMem) -> AssemblerOutputs {
            let r8 = self.convert_gpr_to_assembler_read_write_gpr(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm8 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbb_rm::new(r8.clone(), rm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r8.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_sbbw_rm_raw(&mut self, r16: Gpr, rm16: &GprMem) -> AssemblerOutputs {
            let r16 = self.convert_gpr_to_assembler_read_write_gpr(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm16 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbw_rm::new(r16.clone(), rm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r16.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_sbbl_rm_raw(&mut self, r32: Gpr, rm32: &GprMem) -> AssemblerOutputs {
            let r32 = self.convert_gpr_to_assembler_read_write_gpr(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm32 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbl_rm::new(r32.clone(), rm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r32.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_sbbq_rm_raw(&mut self, r64: Gpr, rm64: &GprMem) -> AssemblerOutputs {
            let r64 = self.convert_gpr_to_assembler_read_write_gpr(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm64 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::sbbq_rm::new(r64.clone(), rm64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r64.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_lock_subb_mi_raw(&mut self, m8: &Amode, imm8: u8) -> AssemblerOutputs {
            let m8 = self.convert_amode_to_assembler_amode(m8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_subb_mi::new(m8.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_subw_mi_raw(&mut self, m16: &Amode, imm16: u16) -> AssemblerOutputs {
            let m16 = self.convert_amode_to_assembler_amode(m16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_subw_mi::new(m16.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_subl_mi_raw(&mut self, m32: &Amode, imm32: u32) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_subl_mi::new(m32.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_subq_mi_sxl_raw(&mut self, m64: &Amode, imm32: i32) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_subq_mi_sxl::new(m64.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_subl_mi_sxb_raw(&mut self, m32: &Amode, imm8: i8) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_subl_mi_sxb::new(m32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_subq_mi_sxb_raw(&mut self, m64: &Amode, imm8: i8) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_subq_mi_sxb::new(m64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_subb_mr_raw(&mut self, m8: &Amode, r8: Gpr) -> AssemblerOutputs {
            let m8 = self.convert_amode_to_assembler_amode(m8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r8 = cranelift_assembler_x64::Gpr::new(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_subb_mr::new(m8.clone(), r8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_subw_mr_raw(&mut self, m16: &Amode, r16: Gpr) -> AssemblerOutputs {
            let m16 = self.convert_amode_to_assembler_amode(m16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_subw_mr::new(m16.clone(), r16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_subl_mr_raw(&mut self, m32: &Amode, r32: Gpr) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_subl_mr::new(m32.clone(), r32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_subq_mr_raw(&mut self, m64: &Amode, r64: Gpr) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_subq_mr::new(m64.clone(), r64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_sbbb_mi_raw(&mut self, m8: &Amode, imm8: u8) -> AssemblerOutputs {
            let m8 = self.convert_amode_to_assembler_amode(m8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_sbbb_mi::new(m8.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_sbbw_mi_raw(&mut self, m16: &Amode, imm16: u16) -> AssemblerOutputs {
            let m16 = self.convert_amode_to_assembler_amode(m16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_sbbw_mi::new(m16.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_sbbl_mi_raw(&mut self, m32: &Amode, imm32: u32) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_sbbl_mi::new(m32.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_sbbq_mi_sxl_raw(&mut self, m64: &Amode, imm32: i32) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_sbbq_mi_sxl::new(m64.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_sbbl_mi_sxb_raw(&mut self, m32: &Amode, imm8: i8) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_sbbl_mi_sxb::new(m32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_sbbq_mi_sxb_raw(&mut self, m64: &Amode, imm8: i8) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_sbbq_mi_sxb::new(m64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_sbbb_mr_raw(&mut self, m8: &Amode, r8: Gpr) -> AssemblerOutputs {
            let m8 = self.convert_amode_to_assembler_amode(m8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r8 = cranelift_assembler_x64::Gpr::new(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_sbbb_mr::new(m8.clone(), r8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_sbbw_mr_raw(&mut self, m16: &Amode, r16: Gpr) -> AssemblerOutputs {
            let m16 = self.convert_amode_to_assembler_amode(m16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_sbbw_mr::new(m16.clone(), r16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_sbbl_mr_raw(&mut self, m32: &Amode, r32: Gpr) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_sbbl_mr::new(m32.clone(), r32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_sbbq_mr_raw(&mut self, m64: &Amode, r64: Gpr) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_sbbq_mr::new(m64.clone(), r64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_subss_a_raw(&mut self, xmm: Xmm, xmm_m32: &XmmMem) -> AssemblerOutputs {
            let xmm = self.convert_xmm_to_assembler_read_write_xmm(xmm); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let xmm_m32 = self.convert_xmm_mem_to_assembler_read_xmm_mem(xmm_m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subss_a::new(xmm.clone(), xmm_m32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let xmm = xmm.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetXmm { inst, xmm }
        }
        fn x64_subsd_a_raw(&mut self, xmm: Xmm, xmm_m64: &XmmMem) -> AssemblerOutputs {
            let xmm = self.convert_xmm_to_assembler_read_write_xmm(xmm); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let xmm_m64 = self.convert_xmm_mem_to_assembler_read_xmm_mem(xmm_m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subsd_a::new(xmm.clone(), xmm_m64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let xmm = xmm.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetXmm { inst, xmm }
        }
        fn x64_subps_a_raw(&mut self, xmm: Xmm, xmm_m128: &XmmMemAligned) -> AssemblerOutputs {
            let xmm = self.convert_xmm_to_assembler_read_write_xmm(xmm); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let xmm_m128 = self.convert_xmm_mem_to_assembler_read_xmm_mem_aligned(xmm_m128); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subps_a::new(xmm.clone(), xmm_m128.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let xmm = xmm.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetXmm { inst, xmm }
        }
        fn x64_subpd_a_raw(&mut self, xmm: Xmm, xmm_m128: &XmmMemAligned) -> AssemblerOutputs {
            let xmm = self.convert_xmm_to_assembler_read_write_xmm(xmm); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let xmm_m128 = self.convert_xmm_mem_to_assembler_read_xmm_mem_aligned(xmm_m128); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::subpd_a::new(xmm.clone(), xmm_m128.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let xmm = xmm.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetXmm { inst, xmm }
        }
        fn x64_xorb_i_raw(&mut self, al: Gpr, imm8: u8) -> AssemblerOutputs {
            let al = self.convert_gpr_to_assembler_fixed_read_write_gpr(al); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorb_i::new(al.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = al.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_xorw_i_raw(&mut self, ax: Gpr, imm16: u16) -> AssemblerOutputs {
            let ax = self.convert_gpr_to_assembler_fixed_read_write_gpr(ax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorw_i::new(ax.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = ax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_xorl_i_raw(&mut self, eax: Gpr, imm32: u32) -> AssemblerOutputs {
            let eax = self.convert_gpr_to_assembler_fixed_read_write_gpr(eax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorl_i::new(eax.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = eax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_xorq_i_sxl_raw(&mut self, rax: Gpr, imm32: i32) -> AssemblerOutputs {
            let rax = self.convert_gpr_to_assembler_fixed_read_write_gpr(rax); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorq_i_sxl::new(rax.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = rax.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_xorb_mi_raw(&mut self, rm8: &GprMem, imm8: u8) -> AssemblerOutputs {
            let rm8 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorb_mi::new(rm8.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm8 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_xorw_mi_raw(&mut self, rm16: &GprMem, imm16: u16) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorw_mi::new(rm16.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_xorl_mi_raw(&mut self, rm32: &GprMem, imm32: u32) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorl_mi::new(rm32.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_xorq_mi_sxl_raw(&mut self, rm64: &GprMem, imm32: i32) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorq_mi_sxl::new(rm64.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_xorl_mi_sxb_raw(&mut self, rm32: &GprMem, imm8: i8) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorl_mi_sxb::new(rm32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_xorq_mi_sxb_raw(&mut self, rm64: &GprMem, imm8: i8) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorq_mi_sxb::new(rm64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_xorb_mr_raw(&mut self, rm8: &GprMem, r8: Gpr) -> AssemblerOutputs {
            let rm8 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r8 = cranelift_assembler_x64::Gpr::new(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorb_mr::new(rm8.clone(), r8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm8 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_xorw_mr_raw(&mut self, rm16: &GprMem, r16: Gpr) -> AssemblerOutputs {
            let rm16 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorw_mr::new(rm16.clone(), r16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm16 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_xorl_mr_raw(&mut self, rm32: &GprMem, r32: Gpr) -> AssemblerOutputs {
            let rm32 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorl_mr::new(rm32.clone(), r32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm32 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_xorq_mr_raw(&mut self, rm64: &GprMem, r64: Gpr) -> AssemblerOutputs {
            let rm64 = self.convert_gpr_mem_to_assembler_read_write_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorq_mr::new(rm64.clone(), r64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            match rm64 {
                asm::GprMem::Gpr(reg) =>  {
                    let gpr = reg.write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:143
                    AssemblerOutputs::RetGpr { inst, gpr }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:144
                }
                asm::GprMem::Mem(_) =>  {
                    AssemblerOutputs::SideEffect { inst }  // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:147
                }
            }
        }
        fn x64_xorb_rm_raw(&mut self, r8: Gpr, rm8: &GprMem) -> AssemblerOutputs {
            let r8 = self.convert_gpr_to_assembler_read_write_gpr(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm8 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorb_rm::new(r8.clone(), rm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r8.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_xorw_rm_raw(&mut self, r16: Gpr, rm16: &GprMem) -> AssemblerOutputs {
            let r16 = self.convert_gpr_to_assembler_read_write_gpr(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm16 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorw_rm::new(r16.clone(), rm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r16.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_xorl_rm_raw(&mut self, r32: Gpr, rm32: &GprMem) -> AssemblerOutputs {
            let r32 = self.convert_gpr_to_assembler_read_write_gpr(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm32 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorl_rm::new(r32.clone(), rm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r32.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_xorq_rm_raw(&mut self, r64: Gpr, rm64: &GprMem) -> AssemblerOutputs {
            let r64 = self.convert_gpr_to_assembler_read_write_gpr(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let rm64 = self.convert_gpr_mem_to_assembler_read_gpr_mem(rm64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorq_rm::new(r64.clone(), rm64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let gpr = r64.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetGpr { inst, gpr }
        }
        fn x64_lock_xorb_mi_raw(&mut self, m8: &Amode, imm8: u8) -> AssemblerOutputs {
            let m8 = self.convert_amode_to_assembler_amode(m8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Imm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_xorb_mi::new(m8.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_xorw_mi_raw(&mut self, m16: &Amode, imm16: u16) -> AssemblerOutputs {
            let m16 = self.convert_amode_to_assembler_amode(m16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm16 = cranelift_assembler_x64::Imm16::new(imm16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_xorw_mi::new(m16.clone(), imm16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_xorl_mi_raw(&mut self, m32: &Amode, imm32: u32) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Imm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_xorl_mi::new(m32.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_xorq_mi_sxl_raw(&mut self, m64: &Amode, imm32: i32) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm32 = cranelift_assembler_x64::Simm32::new(imm32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_xorq_mi_sxl::new(m64.clone(), imm32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_xorl_mi_sxb_raw(&mut self, m32: &Amode, imm8: i8) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_xorl_mi_sxb::new(m32.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_xorq_mi_sxb_raw(&mut self, m64: &Amode, imm8: i8) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let imm8 = cranelift_assembler_x64::Simm8::new(imm8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_xorq_mi_sxb::new(m64.clone(), imm8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_xorb_mr_raw(&mut self, m8: &Amode, r8: Gpr) -> AssemblerOutputs {
            let m8 = self.convert_amode_to_assembler_amode(m8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r8 = cranelift_assembler_x64::Gpr::new(r8); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_xorb_mr::new(m8.clone(), r8.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_xorw_mr_raw(&mut self, m16: &Amode, r16: Gpr) -> AssemblerOutputs {
            let m16 = self.convert_amode_to_assembler_amode(m16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r16 = cranelift_assembler_x64::Gpr::new(r16); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_xorw_mr::new(m16.clone(), r16.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_xorl_mr_raw(&mut self, m32: &Amode, r32: Gpr) -> AssemblerOutputs {
            let m32 = self.convert_amode_to_assembler_amode(m32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r32 = cranelift_assembler_x64::Gpr::new(r32); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_xorl_mr::new(m32.clone(), r32.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_lock_xorq_mr_raw(&mut self, m64: &Amode, r64: Gpr) -> AssemblerOutputs {
            let m64 = self.convert_amode_to_assembler_amode(m64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let r64 = cranelift_assembler_x64::Gpr::new(r64); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::lock_xorq_mr::new(m64.clone(), r64.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            AssemblerOutputs::SideEffect { inst }
        }
        fn x64_xorps_a_raw(&mut self, xmm: Xmm, xmm_m128: &XmmMemAligned) -> AssemblerOutputs {
            let xmm = self.convert_xmm_to_assembler_read_write_xmm(xmm); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let xmm_m128 = self.convert_xmm_mem_to_assembler_read_xmm_mem_aligned(xmm_m128); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorps_a::new(xmm.clone(), xmm_m128.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let xmm = xmm.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetXmm { inst, xmm }
        }
        fn x64_xorpd_a_raw(&mut self, xmm: Xmm, xmm_m128: &XmmMemAligned) -> AssemblerOutputs {
            let xmm = self.convert_xmm_to_assembler_read_write_xmm(xmm); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let xmm_m128 = self.convert_xmm_mem_to_assembler_read_xmm_mem_aligned(xmm_m128); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:102
            let inst = cranelift_assembler_x64::inst::xorpd_a::new(xmm.clone(), xmm_m128.clone()).into(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:109
            let inst = MInst::External { inst }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:113
            let xmm = xmm.as_ref().write.to_reg(); // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:127
            AssemblerOutputs::RetXmm { inst, xmm }
        }
    }; // C:\Users\ACER\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\cranelift-codegen-meta-0.120.0\src\gen_asm.rs:170
}
