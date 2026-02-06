/// Audio visualization module
///
/// This module takes 8-bit audio sample (centered at 128), one sample per clock rise,
/// and produces an 8-bit energy visualization output. 
///
/// To do so, this module compute the RMS energy of the audio signal over a window of 1024 samples, 
/// and outputs the energy level as an 8-bit value. The energy level is computed as follows:
/// 1. For each incoming audio sample, compute the squared difference from the center value (128):
///    energy_sample = (audio_in - 128) * (audio_in - 128)
/// 2. Accumulate the energy samples over a window of 1024 samples:
///    energy_accum += energy_sample
/// 3. After processing 1024 samples, compute the RMS energy:
///    energy_rms = sqrt(energy_accum / 1024)
/// 4. Normalize the RMS energy to fit into an 8-bit value (0-255):
///    viz_out = (energy_rms / max_energy) * 255
///
/// Note: The max_energy is the maximum possible RMS energy, which occurs when all samples are at the maximum deviation from the center (0 or 255).
///       Therefore, max_energy can be computed as:
///       max_energy = sqrt(((255 - 128) * (255 - 128)) * 1024) / 1024) = sqrt(127 * 127) = 127
///
/// To optimize the computation, we avoid to recompute all the square root ops for each samples, instead
/// we use a running sum (ring buffer for the samples, and a accumulator for the energy) to compute the energy in 
/// a more efficient way. In the end, the result is the same, but the implementation is more efficient in logic/multipliers,
/// but it requires more registers to store the intermediate values.
///
/// NOTE: Apparently, professionally, the RMS energy is computed over windows of 300ms, which for a sample rate of 48kHz, 
/// corresponds to 13230 samples. So it costs quite a bit of memory to store the energy samples for such a large window.
/// I tried with 13230 samples but yosys struggled to synthesize the design, so I went with a smaller window of 4096 samples,
/// which is still a good approximation for visualization purposes (and does not make x10 on the synthesis phase).
module audio_viz (
    input clk,
    input rst,
    input [7:0] audio_in,
    output reg [7:0] viz_out
);

    // 1. Make sure he input is signed and ready for being squared
    wire signed [16:0] signed_audio = audio_in;

    // 2. Compute the energy sample (squared difference from the center value)
    reg signed [16:0] centered_audio;
    reg [13:0] energy_sample;
    always @(posedge clk) begin
        centered_audio <= signed_audio - 128;
        // This will be a 16-bit value, since the maximum value is 127*127 = 16129
        energy_sample <= centered_audio * centered_audio; 
    end

    // 3. Accumulate the energy samples over a window of 4096 samples
    reg [25:0] energy_accum = 0; // 26 bits to store the accumulated energy (max (128*128) * (2^12) = ~2^26)
    reg [11:0] samples_head = 0; // 12 bits to count up to 4096 samples
    reg [13:0] energy_samples_buffer [4095:0]; // Buffer to store the last 4096 energy samples

    initial begin
        energy_accum = 0;
        samples_head = 0;
        viz_out = 0;
        // Initialize the energy samples buffer to 0
        for (int i = 0; i < 4096; i++) begin
            energy_samples_buffer[i] = 0;
        end
    end

    always @(posedge clk) begin
        // Subtract the oldest energy sample from the accumulator
        energy_accum <= energy_accum - energy_samples_buffer[samples_head] + energy_sample;
        // Store the new energy sample in the buffer
        energy_samples_buffer[samples_head] <= energy_sample;
        // Move the head of the buffer
        samples_head <= samples_head + 1;
    end

    // 4. Compute the RMS energy and normalize it to fit into an 8-bit value
    always @(posedge clk) begin
        viz_out[0] <= energy_accum[24:12] >= 4; // RMS > 0
        viz_out[1] <= energy_accum[24:12] >= 16; // RMS > 0.125
        viz_out[2] <= energy_accum[24:12] >= 64; // RMS > 0.25
        viz_out[3] <= energy_accum[24:12] >= 256; // RMS > 0.5
        viz_out[4] <= energy_accum[24:12] >= 512; // RMS > 1
        viz_out[5] <= energy_accum[24:12] >= 1024; // RMS > 2
        viz_out[6] <= energy_accum[24:12] >= 2048; // RMS > 4
        viz_out[7] <= energy_accum[24:12] >= 4096; // RMS > 8
    end

endmodule 