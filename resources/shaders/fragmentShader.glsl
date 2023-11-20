#version 410 core
in vec4 TEPosition;
in vec3 TENormal;
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


void main() {
    vec3 TENormal = -normalize(TENormal);
    vec3 lightVector = normalize(lightPos - TEPosition.xyz);
    vec3 V = normalize(cameraPos - TEPosition.xyz);

   // vec3 R = normalize(2 * dot(TENormal,lightVector) * TENormal - lightVector);
    float cos_n_l = dot(TENormal,lightVector);
    float cos_R_l_m = pow(dot(V,reflect(TENormal,lightPos)),m);

   // cos_n_l = max(0.0,min(1.0,cos_n_l));
    cos_R_l_m = max(0.0,min(1.0,cos_R_l_m));
    vec3 rgb = kd * lightColor * objectColor * cos_n_l; + ks * lightColor * objectColor * cos_R_l_m;

    vec2 texCoord = vec2((TEPosition.x + 0.75)/1.5 + (TEPosition.y + 0.75)/1.5);
    FragColor = vec4(rgb,1.0);
}