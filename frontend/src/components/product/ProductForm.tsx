import { useTranslation } from "react-i18next";
import { Product } from "../../domain/entities/Product";
import { useId } from "react";
import { useForm } from "react-hook-form";
import ProductId from "../../domain/valueObjects/ProductId";
import Brand from "../../domain/valueObjects/Brand";
import { ErrorMessage } from "../application/ErrorMessage";
import Quantity from "../../domain/valueObjects/Quantity";
import { UUID } from "../../domain/valueObjects/UUID";
import PlainDate from "../../domain/valueObjects/PlainDate";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTrash } from "@fortawesome/free-solid-svg-icons/faTrash";

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

/** The form's internal representation of a stash item. */
type FormValueStashItem = {
    id: string;
    expiryDate: string;
    quantity: number;
};

/** The form's internal representation of a product. */
type FormValues = {
    id: string;
    brand: string;
    name: string;
    stashItems: Record<string, FormValueStashItem>;
    stashItemOrder: string[];
};

/**
 * Converts a product to the internal representation used by the form.
 *
 * @param product The product to convert.
 *
 * @returns The internal representation of the product.
 */
const productToFormValues = (product: Product): FormValues => ({
    id: product.id.toString(),
    brand: product.brand.toString(),
    name: product.name,
    stashItems: product.stashItems.reduce(
        (r, si) => {
            r[si.id.toString()] = {
                id: si.id.toString(),
                expiryDate: si.expiryDate.toString(),
                quantity: si.quantity.valueOf()
            };
            return r;
        },
        {} as Record<string, FormValueStashItem>
    ),
    stashItemOrder: product.stashItems.map(si => si.id.toString())
});

/**
 * Parses the form's internal representation of a product to a domain product.
 *
 * @param formValues The internal representation of the product.
 *
 * @returns The domain product.
 */
const formValuesToProduct = (formValues: FormValues): Product => ({
    id: new ProductId(formValues.id),
    brand: new Brand(formValues.brand),
    name: formValues.name,
    stashItems: formValues.stashItemOrder.map(id => {
        const si = formValues.stashItems[id];

        return {
            id: new UUID(si.id),
            expiryDate: new PlainDate(si.expiryDate),
            quantity: new Quantity(si.quantity)
        };
    })
});

export const ProductForm = ({ product, productId, onSubmit }: Props): JSX.Element => {
    const { t } = useTranslation();

    const id = useId();

    const form = useForm<FormValues>({
        defaultValues: product
            ? productToFormValues(product)
            : {
                  id: productId?.toString() ?? "",
                  brand: "",
                  name: "",
                  stashItems: {},
                  stashItemOrder: []
              }
    });

    /** Adds a new stash item to the form. */
    const addStashItem = () => {
        const newId = UUID.v4().toString();

        // Update the record
        const stashItems = { ...form.getValues("stashItems") };
        stashItems[newId] = {
            id: newId,
            expiryDate: "",
            quantity: 1
        };
        form.setValue("stashItems", stashItems);

        // Update the order
        const stashItemOrder = [...form.getValues("stashItemOrder")];
        stashItemOrder.push(newId);
        form.setValue("stashItemOrder", stashItemOrder);
    };

    /**
     * Deletes a stash item from the form.
     *
     * @param id The id of the stash item to delete.
     */
    const deleteStashItem = (id: FormValueStashItem["id"]) => {
        // Update the map
        const stashItems = { ...form.getValues("stashItems") };
        delete stashItems.id;
        form.setValue("stashItems", stashItems);

        // Update the order
        const stashItemOrder = form.getValues("stashItemOrder").filter((siId: string) => siId !== id);
        form.setValue("stashItemOrder", stashItemOrder);

        // Unregister the stash item
        //form.unregister(`stashItems.${id}`);
    };

    return (
        <form
            onSubmit={e =>
                void form.handleSubmit(async data => {
                    await onSubmit(formValuesToProduct(data));
                })(e)
            }
        >
            <div>
                <label htmlFor={`${id}-id`}>{t("product:id")}</label>
                <input
                    disabled
                    id={`${id}-id`}
                    type="text"
                    {...form.register("id", {
                        required: t("product:idIsRequired"),
                        validate: (value: string) => {
                            try {
                                new ProductId(value);
                                return true;
                            } catch {
                                return t("product:idIsInvalid");
                            }
                        }
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
                        required: t("product:brandIsRequired"),
                        validate: (value: string) => {
                            try {
                                new Brand(value);
                                return true;
                            } catch {
                                return t("product:brandIsInvalid");
                            }
                        }
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
            <table>
                <thead>
                    <tr>
                        <th>{t("stashItem:expiryDate")}</th>
                        <th>{t("stashItem:quantity")}</th>
                        <th>{t("actions")}</th>
                    </tr>
                </thead>
                <tbody>
                    {form.watch("stashItemOrder").map(id => (
                        <tr key={id}>
                            <td>
                                <input
                                    type="date"
                                    {...form.register(`stashItems.${id}.expiryDate`, {
                                        required: t("stashItem:expiryDateIsRequired"),
                                        validate: (value: string) => {
                                            try {
                                                new PlainDate(value);
                                                return true;
                                            } catch {
                                                return t("stashItem:expiryDateIsInvalid");
                                            }
                                        }
                                    })}
                                />
                                <ErrorMessage>
                                    {form.formState.errors.stashItems?.[id]?.expiryDate?.message}
                                </ErrorMessage>
                            </td>
                            <td>
                                <input
                                    type="number"
                                    min="1"
                                    {...form.register(`stashItems.${id}.quantity`, {
                                        required: t("stashItem:quantityIsRequired"),
                                        min: t("stashItem:quantityMustBeGreaterThanZero"),
                                        valueAsNumber: true,
                                        validate: (value: number) => {
                                            try {
                                                new Quantity(value);
                                                return true;
                                            } catch {
                                                return t("stashItem:quantityIsInvalid");
                                            }
                                        }
                                    })}
                                />
                                <ErrorMessage>{form.formState.errors.stashItems?.[id]?.quantity?.message}</ErrorMessage>
                            </td>
                            <td>
                                <button type="button" onClick={() => deleteStashItem(id)}>
                                    <FontAwesomeIcon icon={faTrash} />
                                </button>
                            </td>
                        </tr>
                    ))}
                </tbody>
                <tfoot>
                    <tr>
                        <td colSpan={3}>
                            <button type="button" onClick={addStashItem}>
                                {t("stashItem:add")}
                            </button>
                        </td>
                    </tr>
                </tfoot>
            </table>
            <button type="submit">{t("save")}</button>
        </form>
    );
};
