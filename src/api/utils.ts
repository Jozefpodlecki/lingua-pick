export const computeDuration = (createdOn: string, currentDate: Date): { seconds: number; hhmmss: string } => {
    const createdOnDate = new Date(createdOn);
    const totalSeconds = Math.floor((currentDate.getTime() - createdOnDate.getTime()) / 1000);

    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;

    const hhmmss = [hours, minutes, seconds]
        .map((unit) => String(unit).padStart(2, "0"))
        .join(":");

    return { seconds: totalSeconds, hhmmss };
};