import { TextField } from "@mui/material";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { DataGrid } from "@mui/x-data-grid";
import { Product } from "../../domain/entities/Product";
import { useQuery } from "@tanstack/react-query";

type Props = {
    getAllProductsWithStashItems: () => Promise<Product[]>;
};

export const Frontpage = ({ getAllProductsWithStashItems }: Props): JSX.Element => {
    const { t } = useTranslation();

    const navigate = useNavigate();

    const [id, setId] = useState<string>("");

    const {
        data: products,
        isLoading,
        error
    } = useQuery({
        queryKey: ["productsWithStashItems"],
        queryFn: getAllProductsWithStashItems
    });

    return (
        <div>
            <form
                className="frontpage"
                onSubmit={e => {
                    e.preventDefault();
                    navigate(`/products/${id}`);
                }}
            >
                <div>
                    <TextField
                        variant="outlined"
                        fullWidth
                        label={t("product:productId")}
                        value={id}
                        onChange={e => {
                            setId(e.target.value);
                        }}
                        autoFocus
                    />
                </div>
            </form>
            {isLoading ? (
                <div>Loading...</div>
            ) : error !== null ? (
                <div>Error: {String(error)}</div>
            ) : (
                <DataGrid
                    rows={products!}
                    columns={[
                        {
                            field: "brand",
                            headerName: t("product:brand")
                        },
                        {
                            field: "name",
                            headerName: t("product:name")
                        },
                        {
                            field: "totalQuantity",
                            headerName: t("productTable:totalQuantity"),
                            valueGetter: params =>
                                params.row.stashItems.reduce((acc, stashItem) => acc + stashItem.quantity.valueOf(), 0)
                        },
                        {
                            field: "nextExpiryDate",
                            headerName: t("productTable:nextExpiryDate"),
                            valueGetter: params =>
                                params.row.stashItems.reduce((acc, stashItem) => {
                                    const expiryDate = stashItem.expiryDate.toISOString();
                                    if (expiryDate < acc) {
                                        return expiryDate;
                                    }
                                    return acc;
                                }, params.row.stashItems[0].expiryDate.toISOString())
                        }
                    ]}
                    initialState={{
                        sorting: {
                            sortModel: [
                                {
                                    field: "nextExpiryDate",
                                    sort: "asc"
                                }
                            ]
                        },
                        pagination: {
                            paginationModel: {
                                page: 0,
                                pageSize: 5
                            }
                        }
                    }}
                    pageSizeOptions={[5, 10, 20]}
                    onRowClick={params => {
                        navigate(`/products/${params.id.toString()}`);
                    }}
                />
            )}
        </div>
    );
};
