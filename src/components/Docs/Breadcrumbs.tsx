import React from "react";
import { ChevronRight, Home } from "lucide-react";
import { Link } from "react-router-dom";

interface BreadcrumbsProps {
    selectedTitle: string;
}

const Breadcrumbs: React.FC<BreadcrumbsProps> = ({ selectedTitle }) => {
    return (
        <nav className="">
            <ul className="flex items-center space-x-2 text-gray-400">
                <li>
                    <Link to="/" className="hover:text-white flex items-center">
                        <Home className="w-5 h-5 mr-1" />
                    </Link>
                </li>
                <li>
                    <ChevronRight className="w-4 h-4" />
                </li>
                <li>
                    <a href="/docs" className="hover:text-white">
                        Documentation
                    </a>
                </li>
            </ul>
        </nav>
    );
};

export default Breadcrumbs;