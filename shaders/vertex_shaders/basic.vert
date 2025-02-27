#version 330 core

layout(location = 0) in vec3 position;

uniform vec2 u_resolution;
uniform float u_time;

out vec4 FragPos;

out vec2 screen_size;

void main() {
    const float PI = 3.14159265359;

    mat3 homothety;
    homothety[0] = vec3(u_time, 0., 0.);
    homothety[1] = vec3(0., u_time, 0.);
    homothety[2] = vec3(0., 0., u_time);

    mat3 rotationy;
    rotationy[0] = vec3(cos(u_time), 0., sin(u_time));
    rotationy[1] = vec3(0., 1, 0.);
    rotationy[2] = vec3(-sin(u_time), 0., cos(u_time));

    mat3 rotationx;
    rotationx[0] = vec3(1., 0., 0.);
    rotationx[1] = vec3(0., cos(u_time), -sin(u_time));
    rotationx[2] = vec3(0., sin(u_time), cos(u_time));

    mat3 rotationz;
    rotationz[0] = vec3(cos(u_time), -sin(u_time), 0.);
    rotationz[1] = vec3(sin(u_time), cos(u_time), 0.);
    rotationz[2] = vec3(0., 0., 1.);

    FragPos = vec4(position, 1.0);

    vec3 uv = position;
    if (u_resolution.x > u_resolution.y) {
        uv.x *= u_resolution.y / u_resolution.x;
    } else {
        uv.y *= u_resolution.x / u_resolution.y;
    } /*
        if (u_time < PI) {
            uv *= 1 - sqrt(1 - pow(u_time / PI, 2));
            gl_Position = vec4(-uv * rotationy * rotationx, 1.0);
        } else {
            gl_Position = vec4(uv, 1.0);
        }*/
}
