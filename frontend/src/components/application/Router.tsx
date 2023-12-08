import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { Frontpage } from "../pages/Frontpage";
import { ProductService } from "../../services/ProductService";
import { useMemo } from "react";
import { ProductPage } from "../pages/ProductPage";

export type Props = {
    productService: ProductService;
};

export const Router = ({ productService }: Props): JSX.Element => {
    const router = useMemo(
        () =>
            createBrowserRouter([
                {
                    path: "/",
                    element: <Frontpage />
                },
                {
                    path: "/products/:id",
                    element: (
                        <ProductPage
                            getProduct={productService.getProduct.bind(productService)}
                            createProduct={async product => {
                                await productService.createProduct(product);
                            }}
                            updateProduct={async product => {
                                await productService.updateProduct(product);
                            }}
                        />
                    )
                }
            ]),
        [productService]
    );

    return <RouterProvider router={router}></RouterProvider>;
};
