import { useTranslation } from "react-i18next";
import { Product } from "../../domain/entities/Product";
import ProductId from "../../domain/valueObjects/ProductId";
import Brand from "../../domain/valueObjects/Brand";
import Quantity from "../../domain/valueObjects/Quantity";
import { UUID } from "../../domain/valueObjects/UUID";
import PlainDate from "../../domain/valueObjects/PlainDate";
import { Controller, useForm } from "react-hook-form";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faTrash } from "@fortawesome/free-solid-svg-icons/faTrash";
import Button from "@mui/material/Button";
import TextField from "@mui/material/TextField";
import Box from "@mui/material/Box";
import TableContainer from "@mui/material/TableContainer";
import Paper from "@mui/material/Paper";
import Table from "@mui/material/Table";
import TableHead from "@mui/material/TableHead";
import TableBody from "@mui/material/TableBody";
import TableRow from "@mui/material/TableRow";
import TableCell from "@mui/material/TableCell";
import TableFooter from "@mui/material/TableFooter";
import { Grid, Typography } from "@mui/material";
import { Link } from "react-router-dom";

export type Props = {
    onSubmit: (product: Product) => void | Promise<void>;
} & (
    | {
          productId?: ProductId;
          product?: never;
      }
    | {
          productId?: never;
          product: Product;
      }
);

/** The form's internal representation of a stash item. */
type FormValueStashItem = {
    id: string;
    expiryDate: string;
    quantity: number;
    isNew: boolean;
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
                quantity: si.quantity.valueOf(),
                isNew: false
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
            quantity: 1,
            isNew: true
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

    const stashItemOrder = form.watch("stashItemOrder");

    return (
        <Box
            component="form"
            onSubmit={e =>
                void form.handleSubmit(async data => {
                    await onSubmit(formValuesToProduct(data));
                })(e)
            }
        >
            <Grid container spacing={3}>
                <Grid item xs={4}>
                    <TextField
                        variant="outlined"
                        fullWidth
                        label={t("product:brand")}
                        error={form.formState.errors.brand?.message !== undefined}
                        helperText={form.formState.errors.brand?.message}
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
                </Grid>
                <Grid item xs={4}>
                    <TextField
                        variant="outlined"
                        fullWidth
                        label={t("product:name")}
                        error={form.formState.errors.name?.message !== undefined}
                        helperText={form.formState.errors.name?.message}
                        {...form.register("name", {
                            required: t("product:nameIsRequired")
                        })}
                    />
                </Grid>
                <Grid item xs={4}>
                    <TextField
                        variant="outlined"
                        fullWidth
                        label={t("product:id")}
                        error={form.formState.errors.id?.message !== undefined}
                        helperText={form.formState.errors.id?.message}
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
                </Grid>
            </Grid>
            <TableContainer component={Paper}>
                <Table>
                    <TableHead>
                        <TableRow>
                            <TableCell sx={{ width: "40%" }}>{t("stashItem:expiryDate")}</TableCell>
                            <TableCell sx={{ width: "40%" }}>{t("stashItem:quantity")}</TableCell>
                            <TableCell sx={{ width: "20%" }}>{t("actions")}</TableCell>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {stashItemOrder.length !== 0 ? (
                            stashItemOrder.map(id => {
                                const stashItem = form.getValues("stashItems")[id];

                                return (
                                    <TableRow key={id}>
                                        <TableCell>
                                            <TextField
                                                type="date"
                                                variant="outlined"
                                                fullWidth
                                                error={
                                                    form.formState.errors.stashItems?.[id]?.expiryDate?.message !==
                                                    undefined
                                                }
                                                helperText={form.formState.errors.stashItems?.[id]?.expiryDate?.message}
                                                disabled={!stashItem.isNew}
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
                                        </TableCell>
                                        <TableCell>
                                            <Controller
                                                control={form.control}
                                                name={`stashItems.${id}.quantity`}
                                                rules={{
                                                    required: t("stashItem:quantityIsRequired"),
                                                    min: {
                                                        value: 1,
                                                        message: t("stashItem:quantityMustBeGreaterThanZero")
                                                    },
                                                    validate: value => {
                                                        try {
                                                            new Quantity(value);
                                                            return true;
                                                        } catch {
                                                            return t("stashItem:quantityIsInvalid");
                                                        }
                                                    }
                                                }}
                                                render={({ field, fieldState: { error } }) => (
                                                    <Box
                                                        display="flex"
                                                        alignItems="stretch"
                                                        justifyContent="center"
                                                        gap="0.5em"
                                                    >
                                                        <Button
                                                            variant="contained"
                                                            type="button"
                                                            onClick={() => field.onChange(field.value - 1)}
                                                        >
                                                            -
                                                        </Button>
                                                        <TextField
                                                            variant="outlined"
                                                            fullWidth
                                                            type="number"
                                                            error={error !== undefined}
                                                            helperText={error?.message}
                                                            {...field}
                                                        />
                                                        <Button
                                                            variant="contained"
                                                            type="button"
                                                            onClick={() => field.onChange(field.value + 1)}
                                                        >
                                                            +
                                                        </Button>
                                                    </Box>
                                                )}
                                            />
                                        </TableCell>
                                        <TableCell>
                                            <Button
                                                variant="contained"
                                                color="warning"
                                                type="button"
                                                onClick={() => deleteStashItem(id)}
                                            >
                                                <FontAwesomeIcon icon={faTrash} /> {t("delete")}
                                            </Button>
                                        </TableCell>
                                    </TableRow>
                                );
                            })
                        ) : (
                            <TableRow>
                                <TableCell colSpan={3}>
                                    <Typography variant="body1">{t("stashItem:noStashItems")}</Typography>
                                </TableCell>
                            </TableRow>
                        )}
                    </TableBody>
                    <TableFooter>
                        <TableRow>
                            <TableCell colSpan={3}>
                                <Button variant="contained" type="button" onClick={addStashItem}>
                                    {t("stashItem:add")}
                                </Button>
                            </TableCell>
                        </TableRow>
                    </TableFooter>
                </Table>
            </TableContainer>
            <Box sx={{ display: "flex", justifyContent: "space-between", marginTop: "1rem" }}>
                <Button variant="contained" color="success" type="submit" disabled={form.formState.isSubmitting}>
                    {t("save")}
                </Button>
                <Button
                    component={Link}
                    variant="contained"
                    color={form.formState.isDirty ? "error" : undefined}
                    to="/"
                    disabled={form.formState.isSubmitting}
                >
                    {t("back")}
                </Button>
            </Box>
        </Box>
    );
};
