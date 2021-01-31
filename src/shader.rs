use shaderc::ShaderKind;

#[allow(dead_code)]
pub enum ShaderQuality {
    Low,
    Medium,
    High,
    Ultra,
}
impl ShaderQuality {
    fn as_str(&self) -> &'static str {
        match *self {
            ShaderQuality::Low => "LOW",
            ShaderQuality::Medium => "MEDIUM",
            ShaderQuality::High => "HIGH",
            ShaderQuality::Ultra => "ULTRA",
        }
    }
}

#[derive(Copy, Clone)]
pub enum ShaderStage {
    EdgeDetectionVS,
    LumaEdgeDetectionPS,

    BlendingWeightVS,
    BlendingWeightPS,

    NeighborhoodBlendingVS,
    NeighborhoodBlendingPS,

    NeighborhoodBlendingAcesTonemapPS,
}
impl ShaderStage {
    fn is_vertex_shader(&self) -> bool {
        match *self {
            ShaderStage::EdgeDetectionVS
            | ShaderStage::BlendingWeightVS
            | ShaderStage::NeighborhoodBlendingVS => true,

            ShaderStage::LumaEdgeDetectionPS
            | ShaderStage::BlendingWeightPS
            | ShaderStage::NeighborhoodBlendingPS
            | ShaderStage::NeighborhoodBlendingAcesTonemapPS => false,
        }
    }
    fn as_str(&self) -> &'static str {
        match *self {
            ShaderStage::EdgeDetectionVS => {
                "layout(location = 0) out float4 offset0;
                 layout(location = 1) out float4 offset1;
                 layout(location = 2) out float4 offset2;
                 layout(location = 3) out float2 texcoord;
                 void main() {
                     if(gl_VertexIndex == 0) gl_Position = vec4(-1, -1, 1, 1);
                     if(gl_VertexIndex == 1) gl_Position = vec4(-1,  3, 1, 1);
        	         if(gl_VertexIndex == 2) gl_Position = vec4( 3, -1, 1, 1);
                     texcoord = gl_Position.xy * vec2(0.5, -0.5) + vec2(0.5);
                     float4 offset[3];
                     SMAAEdgeDetectionVS(texcoord, offset);
                     offset0=offset[0];
                     offset1=offset[1];
                     offset2=offset[2];
                 }"
            }
            ShaderStage::BlendingWeightVS => {
                "layout(location = 0) out float2 pixcoord;
                 layout(location = 1) out float4 offset0;
                 layout(location = 2) out float4 offset1;
                 layout(location = 3) out float4 offset2;
                 layout(location = 4) out float2 texcoord;
                 void main() {
                     if(gl_VertexIndex == 0) gl_Position = vec4(-1, -1, 1, 1);
                     if(gl_VertexIndex == 1) gl_Position = vec4(-1,  3, 1, 1);
        	         if(gl_VertexIndex == 2) gl_Position = vec4( 3, -1, 1, 1);
                     texcoord = gl_Position.xy * vec2(0.5, -0.5) + vec2(0.5);
                     float4 offset[3];
                     SMAABlendingWeightCalculationVS(texcoord, pixcoord, offset);
                     offset0=offset[0];
                     offset1=offset[1];
                     offset2=offset[2];
                 }"
            }
            ShaderStage::NeighborhoodBlendingVS => {
                "layout(location = 0) out float4 offset;
                 layout(location = 1) out float2 texcoord;
                 void main() {
                     if(gl_VertexIndex == 0) gl_Position = vec4(-1, -1, 1, 1);
                     if(gl_VertexIndex == 1) gl_Position = vec4(-1,  3, 1, 1);
        	         if(gl_VertexIndex == 2) gl_Position = vec4( 3, -1, 1, 1);
                     texcoord = gl_Position.xy * vec2(0.5, -0.5) + vec2(0.5);
                     SMAANeighborhoodBlendingVS(texcoord, offset);
                 }"
            }
            ShaderStage::LumaEdgeDetectionPS => {
                "layout(location = 0) in float4 offset0;
                 layout(location = 1) in float4 offset1;
                 layout(location = 2) in float4 offset2;
                 layout(location = 3) in float2 texcoord;
                 layout(set = 0, binding = 1) uniform texture2D colorTex;
                 layout(location = 0) out float2 OutColor;
                 void main() {
                     float4 offset[3] = float4[](offset0, offset1, offset2);
                     OutColor = SMAALumaEdgeDetectionPS(texcoord, offset, colorTex);
                 }"
            }
            ShaderStage::BlendingWeightPS => {
                "layout(location = 0) in float2 pixcoord;
                 layout(location = 1) in float4 offset0;
                 layout(location = 2) in float4 offset1;
                 layout(location = 3) in float4 offset2;
                 layout(location = 4) in float2 texcoord;
                 layout(set = 0, binding = 1) uniform texture2D edgesTex;
                 layout(set = 0, binding = 2) uniform texture2D areaTex;
                 layout(set = 0, binding = 3) uniform texture2D searchTex;
                 layout(location = 0) out float4 OutColor;
                 void main() {
                     vec4 subsampleIndices = vec4(0);
                     float4 offset[3] = float4[](offset0, offset1, offset2);
                     OutColor = SMAABlendingWeightCalculationPS(texcoord, pixcoord, offset,
                         edgesTex, areaTex, searchTex, subsampleIndices);
                 }"
            }
            ShaderStage::NeighborhoodBlendingPS => {
                "layout(location = 0) in float4 offset;
                 layout(location = 1) in float2 texcoord;
                 layout(set = 0, binding = 1) uniform texture2D colorTex;
                 layout(set = 0, binding = 2) uniform texture2D blendTex;
                 layout(location = 0) out float4 OutColor;
                 void main() {
                     OutColor = SMAANeighborhoodBlendingPS(texcoord, offset, colorTex, blendTex);
                 }"
            }
            // See: https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve
            ShaderStage::NeighborhoodBlendingAcesTonemapPS => {
                "layout(location = 0) in float4 offset;
                 layout(location = 1) in float2 texcoord;
                 layout(set = 0, binding = 1) uniform texture2D colorTex;
                 layout(set = 0, binding = 1) uniform texture2D blendTex;
                 layout(location = 0) out float4 OutColor;
                 void main() {
                     float a = 2.51f;
                     float b = 0.03f;
                     float c = 2.43f;
                     float d = 0.59f;
                     float e = 0.14f;
                     OutColor = SMAANeighborhoodBlendingPS(texcoord, offset, colorTex, blendTex);
                     vec3 x = OutColor.rgb;
                     OutColor.rgb = clamp((x*(a*x+b))/(x*(c*x+d)+e), vec3(0), vec3(1));
                 }"
            }
        }
    }
}

pub(crate) struct ShaderSource {
    pub width: u32,
    pub height: u32,
    pub quality: ShaderQuality,
}
impl ShaderSource {
    fn get_stage(&self, stage: ShaderStage) -> String {
        format!(
            "#version 450 core
            #extension GL_EXT_samplerless_texture_functions: require
            #define SMAA_RT_METRICS float4(1.0 / {0}.0, 1.0 / {1}.0, {0}.0, {1}.0)
            #define SMAA_GLSL_3
            #define SMAA_PRESET_{2}
            #define SMAA_INCLUDE_{3} 0
            layout(set = 0, binding = 0) uniform sampler linearSampler;
            {4}
            {5}",
            self.width,
            self.height,
            self.quality.as_str(),
            if stage.is_vertex_shader() { "PS" } else { "VS" },
            include_str!("../third_party/smaa/SMAA.hlsl"),
            stage.as_str(),
        )
    }
    pub fn get_shader(
        &self,
        device: &wgpu::Device,
        stage: ShaderStage,
        name: &'static str,
    ) -> Result<wgpu::ShaderModule, failure::Error> {
        let source = self.get_stage(stage);
        let mut glsl_compiler = shaderc::Compiler::new().unwrap();
        let spirv = glsl_compiler
            .compile_into_spirv(
                &source,
                if stage.is_vertex_shader() {
                    ShaderKind::Vertex
                } else {
                    ShaderKind::Fragment
                },
                name,
                "main",
                None,
            )?
            .as_binary()
            .to_vec();

        Ok(device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some(name),
            source: wgpu::ShaderSource::SpirV(spirv.into()),
            flags: wgpu::ShaderFlags::empty(),
        }))
    }
}
