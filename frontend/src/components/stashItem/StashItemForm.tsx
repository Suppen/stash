import { useForm } from "react-hook-form";
import { StashItem } from "../../domain/entities/StashItem";
import Quantity from "../../domain/valueObjects/Quantity";
import PlainDate from "../../domain/valueObjects/PlainDate";
import { useId } from "react";
import { UUID } from "../../domain/valueObjects/UUID";
import { ErrorMessage } from "../application/ErrorMessage";
import { useTranslation } from "react-i18next";

export type Props = {
    stashItem?: StashItem;
    onSubmit: (stashItem: StashItem) => void | Promise<void>;
};

export const StashItemForm = ({ stashItem, onSubmit }: Props): JSX.Element => {
    const { t } = useTranslation();

    const id = useId();

    const form = useForm({
        defaultValues: {
            quantity: stashItem?.quantity.value() ?? 1,
            expiry_date: stashItem?.expiryDate.toISOString() ?? undefined
        }
    });

    return (
        <form
            onSubmit={e =>
                void form.handleSubmit(async data => {
                    const output: StashItem = {
                        id: stashItem?.id ?? UUID.v4(),
                        quantity: new Quantity(Number(data.quantity)),
                        expiryDate: new PlainDate(data.expiry_date!)
                    };

                    await onSubmit(output);
                })(e)
            }
        >
            <div>
                <label htmlFor={`${id}-quantity`}>{t("stashItem:quantity")}</label>
                <input
                    id={`${id}-quantity`}
                    type="number"
                    {...form.register("quantity", {
                        required: t("stashItem:quantityIsRequired"),
                        min: {
                            value: 1,
                            message: t("stashItem:quantityMustBeGreaterThanZero")
                        }
                    })}
                />
                <ErrorMessage>{form.formState.errors.quantity?.message}</ErrorMessage>
            </div>
            <div>
                <label htmlFor={`${id}-expiry_date`}>{t("stashItem:expiryDate")}</label>
                <input
                    id={`${id}-expiry_date`}
                    type="date"
                    {...form.register("expiry_date", {
                        required: t("stashItem:expiryDateIsRequired")
                    })}
                />
                <ErrorMessage>{form.formState.errors.expiry_date?.message}</ErrorMessage>
            </div>
            <button type="submit">{t("save")}</button>
        </form>
    );
};
