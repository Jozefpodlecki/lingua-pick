import React from "react";
import { Link } from "react-router-dom";
import { QuizSession } from "../../models";
import ReactECharts from "echarts-for-react";

interface Props {
    session: QuizSession;
}

const Completed: React.FC<Props> = ({ session }) => {
    const correctAnswers = session.exercises.filter((ex) => ex.isCorrect).length;
    const incorrectAnswers = session.exercises.filter((ex) => ex.isCompleted && !ex.isCorrect).length;

    const chartOptions = {
        title: {
            text: "Session Results",
            left: "center",
            textStyle: {
                color: "#ffffff",
            },
        },
        tooltip: {
            trigger: "axis",
        },
        xAxis: {
            type: "category",
            data: ["Correct", "Incorrect"],
            axisLine: {
                lineStyle: {
                    color: "#ffffff",
                },
            },
        },
        yAxis: {
            type: "value",
            axisLine: {
                lineStyle: {
                    color: "#ffffff",
                },
            },
        },
        series: [
            {
                data: [correctAnswers, incorrectAnswers],
                type: "bar",
                itemStyle: {
                    color: (params: any) => (params.dataIndex === 0 ? "#4caf50" : "#f44336"),
                },
            },
        ],
        backgroundColor: "#000000",
    };

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-black text-white">
            <h1 className="text-4xl font-bold mb-4">Session Completed!</h1>
            <p className="text-lg mb-8">Congratulations on completing the session!</p>
            <div className="w-full max-w-lg mb-8">
                <ReactECharts option={chartOptions} style={{ height: "400px", width: "100%" }} />
            </div>
            <div className="flex space-x-4">
                <Link
                    to="/"
                    className="bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded"
                >
                    Go to Home
                </Link>
            </div>
        </div>
    );
};

export default Completed;