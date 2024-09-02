import { IShop } from "@store/shops";

export interface IShopProps {
  info: IShop;
}

export const Shop = (props: IShopProps) => {
  return (
    <div class="flex flex-col border border-gray-115 rounded-3xl p-6 gap-6">
      <div class="flex gap-4 items-start">
        <img src={props.info.iconBase64Src} class="rounded-full" />
        <div class="flex flex-col gap-2">
          <p class="font-semibold text-white text-lg">{props.info.name}</p>
          <p class="font-normal text-gray-140 text-sm">{props.info.description}</p>
        </div>
      </div>
    </div>
  );
};
