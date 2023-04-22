import sampleComputeShaderSource from "./shaders/sampleCompute.wgsl?raw";
import {} from "gl-matrix";

export async function setupContext({ canvas }: { canvas: HTMLCanvasElement }) {
  const adapter = await navigator.gpu.requestAdapter();
  if (!adapter) {
    throw new Error("adapter not found");
  }
  const device = await adapter.requestDevice();
  if (!device) {
    throw new Error("device not found");
  }

  const context = canvas.getContext("webgpu");
  if (!context) {
    throw new Error("could not create webgpu context");
  }
  const presentationFormat = navigator.gpu.getPreferredCanvasFormat();
  const devicePixelRatio = window.devicePixelRatio || 1;
  canvas.width = canvas.clientWidth * devicePixelRatio;
  canvas.height = canvas.clientHeight * devicePixelRatio;
  context.configure({
    device,
    format: presentationFormat,
    alphaMode: "opaque",
  });
  return { adapter, context, device };
}

function randomMatrix(rows: number, cols: number) {
  const elements = [];
  for (let i = 0; i < rows * cols; ++i) {
    elements.push(Math.random() * 10);
  }
  return new Float32Array([rows, cols, ...elements]);
}

function printMatrix(matrix: Float32Array, name?: string) {
  const rows = matrix[0];
  const cols = matrix[1];
  const label = name ? ` -- ${name}` : "";
  let msg = `${rows}x${cols}${label}` + "\n";
  const elements = matrix.slice(2);
  for (let j = 0; j < rows; ++j) {
    const startIndex = j * cols;
    const endIndex = startIndex + cols;
    const slice = elements.slice(startIndex, endIndex);
    msg += slice.toString() + "\n";
  }
  console.log(msg);
}

export async function webgpuTsDemo({
  canvas,
  messageElt,
}: {
  canvas: HTMLCanvasElement;
  messageElt: HTMLElement;
}) {
  const {  adapter: _adapter, context:_context, device } = await setupContext({ canvas });
  const firstMatrix = randomMatrix(10, 4);
  const secondMatrix = randomMatrix(4, 10);
  printMatrix(firstMatrix, 'Matrix A');
  printMatrix(secondMatrix), 'Matrix B';
  const bufferFirstMatrix = device.createBuffer({
    mappedAtCreation: true,
    size: firstMatrix.byteLength,
    usage: GPUBufferUsage.STORAGE,
  });
  const arrayBufferFirstMatrix = bufferFirstMatrix.getMappedRange();
  new Float32Array(arrayBufferFirstMatrix).set(firstMatrix);
  bufferFirstMatrix.unmap();

  const bufferSecondMatrix = device.createBuffer({
    mappedAtCreation: true,
    size: secondMatrix.byteLength,
    usage: GPUBufferUsage.STORAGE,
  });
  const arrayBufferSecondMatrix = bufferSecondMatrix.getMappedRange();
  new Float32Array(arrayBufferSecondMatrix).set(secondMatrix);
  bufferSecondMatrix.unmap();

  const resultMatrixBufferSize =
    Float32Array.BYTES_PER_ELEMENT * (2 + firstMatrix[0] * secondMatrix[1]);
  const resultMatrixBuffer = device.createBuffer({
    size: resultMatrixBufferSize,
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC,
  });

  const bindGroupLayout = device.createBindGroupLayout({
    entries: [
      {
        binding: 0,
        visibility: GPUShaderStage.COMPUTE,
        buffer: {
          type: "read-only-storage",
        },
      },
      {
        binding: 1,
        visibility: GPUShaderStage.COMPUTE,
        buffer: {
          type: "read-only-storage",
        },
      },
      {
        binding: 2,
        visibility: GPUShaderStage.COMPUTE,
        buffer: {
          type: "storage",
        },
      },
    ],
  });

  const bindGroup = device.createBindGroup({
    layout: bindGroupLayout,
    entries: [
      {
        binding: 0,
        resource: {
          buffer: bufferFirstMatrix,
        },
      },
      {
        binding: 1,
        resource: {
          buffer: bufferSecondMatrix,
        },
      },
      {
        binding: 2,
        resource: {
          buffer: resultMatrixBuffer,
        },
      },
    ],
  });
  const shaderModule = device.createShaderModule({
    code: sampleComputeShaderSource,
  });

  const computePipeline = device.createComputePipeline({
    layout: device.createPipelineLayout({
      bindGroupLayouts: [bindGroupLayout],
    }),
    compute: {
      module: shaderModule,
      entryPoint: "main",
    },
  });

  const commandEncoder = device.createCommandEncoder();

  const passEncoder = commandEncoder.beginComputePass();
  passEncoder.setPipeline(computePipeline);
  passEncoder.setBindGroup(0, bindGroup);
  const workgroupCountX = Math.ceil(firstMatrix[0] / 8);
  const workgroupCountY = Math.ceil(secondMatrix[1] / 8);
  passEncoder.dispatchWorkgroups(workgroupCountX, workgroupCountY);
  passEncoder.end();

  // Get a GPU buffer for reading in an unmapped state.
  const gpuReadBuffer = device.createBuffer({
    size: resultMatrixBufferSize,
    usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
  });

  // Encode commands for copying buffer to buffer.
  commandEncoder.copyBufferToBuffer(
    resultMatrixBuffer /* source buffer */,
    0 /* source offset */,
    gpuReadBuffer /* destination buffer */,
    0 /* destination offset */,
    resultMatrixBufferSize /* size */
  );

  // Submit GPU commands.
  const gpuCommands = commandEncoder.finish();
  device.queue.submit([gpuCommands]);

  // Read buffer.
  await gpuReadBuffer.mapAsync(GPUMapMode.READ);
  const arrayBuffer = gpuReadBuffer.getMappedRange();
  printMatrix(new Float32Array(arrayBuffer), "result");
}
