macro_rules! declare_bitpacker {
    ($cpufeature:meta) => {
        declare_bitpacker_common!(
            $cpufeature,
            struct DeltaIntegrate {
                current: DataType,
                output_ptr: *mut DataType,
            }

            impl DeltaIntegrate {
                unsafe fn new(initial: u32, output_ptr: *mut DataType) -> DeltaIntegrate {
                    DeltaIntegrate {
                        current: set1(initial as i32),
                        output_ptr,
                    }
                }
            }

            impl Sink for DeltaIntegrate {
                #[inline]
                unsafe fn process(&mut self, delta: DataType) {
                    self.current = integrate_delta(self.current, delta);
                    store_unaligned(self.output_ptr, self.current);
                    self.output_ptr = self.output_ptr.add(1);
                }
            }
        );
    };
}
