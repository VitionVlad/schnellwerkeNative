sh:
	glslc shaders/shader.frag -o shaders/frag
	glslc shaders/shader.vert -o shaders/vert
	glslc shaders/shadow.vert -o shaders/shadow
	glslc shaders/deffered.frag -o shaders/fdeffered
	glslc shaders/deff_qo.frag -o shaders/fdeffqo
	glslc shaders/deff_em.frag -o shaders/fdeffem
	glslc shaders/deffered.vert -o shaders/vdeffered
	glslc shaders/text.frag -o shaders/ftext
	glslc shaders/pltx.frag -o shaders/pltx