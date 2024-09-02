import { CreateShopForm } from "@components/create-shop-form";
import { Page } from "@components/page";
import { useLocation } from "@solidjs/router";

export const RegisterShopPage = () => {
  const { query } = useLocation();

  const id = () => (query["id"] ? BigInt(query["id"]) : undefined);

  return (
    <Page slim>
      <CreateShopForm id={id()} />
    </Page>
  );
};
