import { useTranslation } from "react-i18next";
import { Product } from "../../domain/entities/Product";
import { useId } from "react";
import { useForm } from "react-hook-form";
import ProductId from "../../domain/valueObjects/ProductId";
import Brand from "../../domain/valueObjects/Brand";
import { ErrorMessage } from "../application/ErrorMessage";

export type Props = {
    onSubmit: (product: Product) => void | Promise<void>;
} & (
    | {
          productId?: ProductId;
          product?: never;
      }
    | {
          productId?: never;
          product?: Product;
      }
);

export const ProductForm = ({ product, productId, onSubmit }: Props): JSX.Element => {
    const { t } = useTranslation();

    const id = useId();

    const form = useForm({
        defaultValues: {
            id: product?.id.toString() ?? productId?.toString() ?? "",
            brand: product?.brand.toString() ?? "",
            name: product?.name ?? ""
        }
    });

    return (
        <form
            onSubmit={e =>
                void form.handleSubmit(async data => {
                    const output: Product = {
                        id: new ProductId(data.id),
                        brand: new Brand(data.brand),
                        name: data.name,
                        stashItems: product?.stashItems ?? []
                    };

                    await onSubmit(output);
                })(e)
            }
        >
            <div>
                <label htmlFor={`${id}-id`}>{t("product:id")}</label>
                <input
                    id={`${id}-id`}
                    type="text"
                    {...form.register("id", {
                        required: t("product:idIsRequired")
                    })}
                />
                <ErrorMessage>{form.formState.errors.id?.message}</ErrorMessage>
            </div>
            <div>
                <label htmlFor={`${id}-brand`}>{t("product:brand")}</label>
                <input
                    id={`${id}-brand`}
                    type="text"
                    {...form.register("brand", {
                        required: t("product:brandIsRequired")
                    })}
                />
                <ErrorMessage>{form.formState.errors.brand?.message}</ErrorMessage>
            </div>
            <div>
                <label htmlFor={`${id}-name`}>{t("product:name")}</label>
                <input
                    id={`${id}-name`}
                    type="text"
                    {...form.register("name", {
                        required: t("product:nameIsRequired")
                    })}
                />
                <ErrorMessage>{form.formState.errors.name?.message}</ErrorMessage>
            </div>
            <button type="submit">{t("product:save")}</button>
        </form>
    );
};
