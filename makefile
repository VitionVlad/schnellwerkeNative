sh:
	glslc shaders/shader.frag -o shaders/frag
	glslc shaders/shader.vert -o shaders/vert
	glslc shaders/shadow.vert -o shaders/shadow
	glslc shaders/deffered.frag -o shaders/fdeffered
	glslc shaders/deffered.vert -o shaders/vdeffered
	glslc shaders/text.frag -o shaders/ftext
	glslc shaders/image.frag -o shaders/fimg
	glslc shaders/light.frag -o shaders/flight
	glslc shaders/simplelight.frag -o shaders/fslight