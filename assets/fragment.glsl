#version 120

precision highp float;
uniform mat4 uMVPMatrix;
uniform vec2 uWindowSize;

void main() {
	//Scale point by input transformation matrix
	vec2 p = (uMVPMatrix * vec4(gl_FragCoord.xy / uWindowSize, 0, 1)).xy;
	vec2 c = p;

	//Set default color to HSV value for black
	vec3 color=vec3(0.0,0.0,0.0);

	//Max number of iterations will arbitrarily be defined as 100. Finer detail with more computation will be found for larger values.
	for(int i=0;i<300;i++){
		//Perform complex number arithmetic
		p= vec2(p.x*p.x-p.y*p.y,2.0*p.x*p.y)+c;
		
		if (dot(p,p)>4.0){
			//The point, c, is not part of the set, so smoothly color it. colorRegulator increases linearly by 1 for every extra step it takes to break free.
			float colorRegulator = float(i-1)-log(((log(dot(p,p)))/log(2.0)))/log(2.0);
			//This is a coloring algorithm I found to be appealing. Written in HSV, many functions will work.
			color = vec3(0.95 + .012*colorRegulator , 1.0, .2+.4*(1.0+sin(.3*colorRegulator)));
			break;
		}
	}
	//Change color from HSV to RGB. Algorithm from https://gist.github.com/patriciogonzalezvivo/114c1653de9e3da6e1e3
	vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
	vec3 m = abs(fract(color.xxx + K.xyz) * 6.0 - K.www);
	gl_FragColor.rgb = color.z * mix(K.xxx, clamp(m - K.xxx, 0.0, 1.0), color.y);

	gl_FragColor.a=1.0;
}
