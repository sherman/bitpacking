macro_rules! declare_bitpacker_avx2 {
    ($cpufeature:meta) => {
        declare_bitpacker_common!(
            $cpufeature,
            struct DeltaIntegrate {
                base: u32,
                current: DataType,
                output_ptr: *mut DataType,
            }

            impl DeltaIntegrate {
                unsafe fn new(initial: u32, output_ptr: *mut DataType) -> DeltaIntegrate {
                    DeltaIntegrate {
                        base: initial,
                        current: set1(initial as i32),
                        output_ptr,
                    }
                }
            }

            impl Sink for DeltaIntegrate {
                #[inline]
                unsafe fn process(&mut self, delta: DataType) {
                    // 1) local prefix sums of deltas (relative to 0)
                    let local_prefix_sum_vector = scan8_hillis_steele_i32(delta);

                    // 2) add scalar base to all lanes (wrapping in 32-bit)
                    let base_vector = set1(self.base as i32);
                    let out_vector = add(local_prefix_sum_vector, base_vector);

                    // 3) store
                    store_unaligned(self.output_ptr, out_vector);
                    self.output_ptr = self.output_ptr.add(1);

                    // 4) base += sum(delta_chunk) == last(pref)
                    self.base = self.base.wrapping_add(last_lane_u32(local_prefix_sum_vector))
                }
            }
        );
    };
}
