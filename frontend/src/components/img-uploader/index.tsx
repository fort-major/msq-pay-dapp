import { Btn } from "@components/btn";
import { EIconKind, Icon } from "@components/icon";
import { ValidationError } from "@components/validation-error";
import { COLORS } from "@utils/colors";
import { err, ErrorCode } from "@utils/error";
import { eventHandler } from "@utils/security";
import { Result } from "@utils/types";
import { createSignal, onMount, Show } from "solid-js";

export type TImgUploaderValidation = { required: null } | { maxSizeBytes: number };

export interface IImgUploaderProps {
  onChange: (fileB64: Result<string | undefined>) => void;
  validations?: TImgUploaderValidation[];
}

export const ImgUploader = (props: IImgUploaderProps) => {
  const [error, setError] = createSignal<string | undefined>();
  const [fileName, setFileName] = createSignal<string | undefined>();
  const [id, setId] = createSignal(0);

  onMount(() => {
    setId(Date.now());
  });

  const handleChange = eventHandler(async (e: Event & { target: HTMLInputElement }) => {
    if (!e.target.files) {
      err(ErrorCode.UNREACHEABLE, "Event should contain files");
    }

    const file: File | undefined = e.target.files[0];
    setFileName(file?.name);

    const reader = new FileReader();

    reader.onload = () => {
      const base64String: string = reader.result as string;

      setError(isValid(file, base64String, props.validations));

      props.onChange(error() ? Result.Err(base64String) : Result.Ok(base64String));
    };

    reader.readAsDataURL(file);
  });

  return (
    <div class="flex flex-col gap-2">
      <label class="cursor-pointer flex flex-row gap-2 justify-between items-center" for={`img-upload-${id()}`}>
        <div class="flex font-semibold flex-nowrap items-center justify-center gap-2 px-5 py-2 rounded-full h-[40px] bg-orange">
          <span>Select a file</span>
        </div>
        <p class="font-normal text-gray-190 text-md">
          <Show when={fileName()} fallback={"File not selected"}>
            {fileName()}
          </Show>
        </p>
        <input class="hidden" type="file" id={`img-upload-${id()}`} onChange={handleChange} />
      </label>
      <ValidationError error={error()} />
    </div>
  );
};

const validImageTypes = ["image/jpeg", "image/png", "image/svg", "image/jpg"];

function isValid(file?: File, base64?: string, validations?: TImgUploaderValidation[]): string | undefined {
  if (!validations) return undefined;

  for (let validation of validations) {
    if ("required" in validation && !file) {
      return "The image is required";
    }
  }

  if (!file || !base64) return undefined;

  if (!validImageTypes.includes(file.type)) {
    return `Supported extensions are: ${validImageTypes.map((it) => `'${it.replace("image/", ".")}'`).join(", ")}`;
  }

  for (let validation of validations) {
    if ("maxSizeBytes" in validation && base64.length > validation.maxSizeBytes) {
      return `Image size is ${(base64.length / 1024).toFixed(2)}kB, max size is ${(
        validation.maxSizeBytes / 1024
      ).toFixed(2)}kB`;
    }
  }

  return undefined;
}
