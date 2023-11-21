#version 410 core
in vec4 TEPosition;
in vec3 TENormal;
in vec2 TexCoord;
in mat3 TBN;
out vec4 FragColor;

uniform vec3 ourColor;
uniform vec3 lightPos;
uniform float ks;
uniform float kd;
uniform uint m;
uniform vec3 lightColor;
uniform vec3 objectColor;
uniform vec3 cameraPos;
uniform sampler2D ourTexture;
uniform sampler2D normalMap;
uniform bool isTextureSet;
uniform bool isNormalMapSet;

void main() {
   vec3 normal = vec3(0,0,1);
   if(isNormalMapSet) normal = normalize(texture(normalMap, TexCoord).rgb * 2.0 -1.0);

    normal = -normalize(TBN * normal);

    vec3 TENormal = -normalize(TENormal);
    vec3 lightVector = normalize(lightPos - TEPosition.xyz);
    vec3 V = normalize(cameraPos - TEPosition.xyz);

    float cos_n_l = dot(normal,lightVector);
    float cos_R_l_m = pow(dot(V,reflect(normal,lightPos)),m);

    cos_R_l_m = max(0.0,min(1.0,cos_R_l_m));
    vec3 rgb = kd * lightColor * objectColor * cos_n_l + ks * lightColor * objectColor * cos_R_l_m;

    vec4 normal_text = texture(normalMap,TexCoord);
    vec4 tex = texture(ourTexture,TexCoord);
    FragColor = vec4(rgb,1.0);
    if(isTextureSet) FragColor = tex * vec4(rgb,1.0);

}