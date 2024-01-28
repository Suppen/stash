import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useParams } from "react-router-dom";
import { Product } from "../../domain/entities/Product";
import ProductId from "../../domain/valueObjects/ProductId";
import { ProductForm } from "../product/ProductForm";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { Box, Typography } from "@mui/material";
import { toast } from "react-toastify";

export type Props = {
    getProduct: (productId: ProductId) => Promise<Product | null>;
    createProduct: (product: Product) => Promise<void>;
    updateProduct: (product: Product) => Promise<void>;
};

export const ProductPage = ({ getProduct, createProduct, updateProduct }: Props): JSX.Element => {
    const { t } = useTranslation();
    const queryClient = useQueryClient();

    // Get the ID from the URL
    const { id } = useParams<{ id: string }>();

    if (id === undefined) {
        // If the ID is not there, a developer fucked up
        throw new Error("id is undefined");
    }

    const productId = useMemo(() => new ProductId(id), [id]);

    // Get the product
    const {
        data: product,
        isLoading,
        error
    } = useQuery({
        queryKey: ["product", id],
        queryFn: () => getProduct(productId)
    });

    const invalidateQueries = () => Promise.all([queryClient.invalidateQueries({ queryKey: ["product", id] })]);

    if (error !== null) {
        // TODO Proper error handling
        console.error(error);
        return <p>Something went wrong</p>;
    }

    if (isLoading || product === undefined) {
        return <p>Loading...</p>;
    }

    return (
        <Box>
            {product === null ? (
                <>
                    <Typography variant="h3">{t("product:newProduct")}</Typography>
                    <ProductForm
                        productId={productId}
                        onSubmit={async product => {
                            try {
                                await createProduct(product);
                                toast.success(t("product:productCreated"));
                                await invalidateQueries();
                            } catch (e) {
                                console.error(e);
                                toast.error(t("unknownErrorOccurred"));
                            }
                        }}
                    />
                </>
            ) : (
                <>
                    <Typography variant="h3">{t("product:updateProduct")}</Typography>
                    <ProductForm
                        product={product}
                        onSubmit={async product => {
                            try {
                                await updateProduct(product);
                                toast.success(t("product:productUpdated"));
                                await invalidateQueries();
                            } catch (e) {
                                console.error(e);
                                toast.error(t("unknownErrorOccurred"));
                            }
                        }}
                    />
                </>
            )}
        </Box>
    );
};
