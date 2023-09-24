
pub(crate) const DEFAULT_FRAGMENT_SHADER: &'static str = "#version 100
precision highp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    gl_FragColor = texture2D(Texture, uv);
}
";

pub(crate) const DEFAULT_VERTEX_SHADER: &'static str = "#version 100
precision highp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;
uniform mat4 Move;
//uniform vec3 Translation;


void main() {
    gl_Position = Projection * Model * Move * vec4(position, 1) ;    
    uv = texcoord;
}
";



pub(crate) const STARS_FRAGMENT_SHADER: &'static str = "#version 100
precision highp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    gl_FragColor = texture2D(Texture, uv);
}
";

pub(crate) const STARS_VERTEX_SHADER: &'static str = "#version 100
precision highp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;
//uniform vec3 Translation;


void main() {
    gl_Position = Projection * Model * vec4(position, 1) ;    
    uv = texcoord;
}
";

