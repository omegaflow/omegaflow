const capacity = device.limits.maxStorageBufferBindingSize;
const model_n_max = data.n_max;
const hardware_n_max = capacity >= 73728 ? 133 : capacity >= 1456 ? 12 : 8;
const nMax = Math.min(model_n_max, hardware_n_max);
const legendreSize = nMax == 133 ? 9045 : nMax == 12 ? 105 : 45;
const tier = nMax == 133 ? 2 : nMax == 12 ? 1 : 0;

const pipe=device.createRenderPipeline({layout:pl,vertex:{module:sm,entryPoint:'vs'},fragment:{module:sm,entryPoint:'fs',targets:[{format:fmt}]},primitive:{topology:'triangle-list'},constants:{"HARDWARE_TIER":tier,"N_MAX":nMax,"LEGENDRE_ARRAY_SIZE":legendreSize}});
