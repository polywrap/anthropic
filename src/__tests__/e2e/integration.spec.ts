import { PolywrapClient, ClientConfigBuilder } from "@polywrap/client-js";
import * as App from "../types/wrap";
import path from "path";

jest.setTimeout(60000);

describe("Anthropic API", () => {

  let wrapperUri: string;
  let client: PolywrapClient;

  const HUMAN_PROMPT = "\n\nHuman:";
  const AI_PROMPT = "\n\nAssistant:";

  beforeAll(() => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;

    const config = new ClientConfigBuilder()
      .addDefaults()
      .addEnv(wrapperUri, { api_key: "" })
      .build()

    client = new PolywrapClient(config)
  })

  it("basic sync", async () => {
    const params: App.Anthropic_SamplingParameters = {
      prompt: `${HUMAN_PROMPT} How many toes do dogs have?${AI_PROMPT}`,
      stopSequences: [HUMAN_PROMPT],
      maxTokensToSample: 200,
      model: "claude-v1",
    }

    const result = await client.invoke<App.Anthropic_CompletionResponse>({
      uri: wrapperUri,
      method: "complete",
      args: { params }
    });

    if (!result.ok) throw result.error;

    console.log(JSON.stringify(result.value, null, 2))

    expect(result.value.completion).toBeTruthy()
    expect(result.value.stop).toBeFalsy()
    expect(result.value.stopReason).toBeTruthy()
    expect(result.value.truncated).toBeFalsy()
    expect(result.value.exception).toBeFalsy()
    expect(result.value.logId).toBeTruthy()
  });
});
